use log::trace;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::cost;

/// Structs for deserializing the OpenAI Chat Completions response.
#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessageResponse,
}

#[derive(Deserialize)]
pub struct ChatMessageResponse {
    pub content: String,
}

/// Struct for the commit message format
#[derive(Debug, Deserialize, Serialize)]
pub struct CommitMessage {
    pub summary: String,
    pub description: String,
}

/// Generates a commit message by sending the provided diff to the OpenAI ChatGPT API.
/// If dry_run is true, it will trace the request instead of making an actual API call.
/// Returns a CommitMessage struct containing the summary and description.
pub fn generate_commit_message(
    diff: &str,
    dry_run: bool,
) -> Result<CommitMessage, Box<dyn std::error::Error>> {
    if dry_run {
        trace!(
            "[DRY RUN] Would send request to OpenAI API with diff: {}",
            diff
        );
        // Return a mock commit message in dry-run mode
        return Ok(CommitMessage {
            summary: "Dry run commit message".to_string(),
            description: "This is a mock commit message generated in dry run mode.".to_string(),
        });
    }

    // Get API key from environment
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    // Updated prompt to request JSON format
    let prompt = format!(
        r#"Generate a Git commit message for the following changes.
        Respond with a JSON object containing:
        - 'summary': A concise summary (max 64 characters)
        - 'description': A detailed description (or empty string if not needed)

        Format:
        {{
          "summary": "...",
          "description": "..."
        }}

        Changes to analyze:
        {}
        "#,
        diff
    );

    // Estimate cost before proceeding
    let model = "gpt-4"; // Default model
    let (token_count, estimated_cost) = cost::estimate_cost(&prompt, model);

    // Prompt user for confirmation
    if !cost::prompt_for_confirmation(token_count, estimated_cost) {
        return Err("User canceled the operation".into());
    }

    // Build the JSON request body.
    let request_body = json!({
       "model": model,
       "messages": [
            { "role": "user", "content": prompt }
       ]
    });

    // Send the POST request to the OpenAI Chat Completions API.
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()?;

    // Check if the API call was successful.
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    // Deserialize the JSON response.
    let json_response: ChatResponse = response.json()?;
    let content = json_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .ok_or("No response from API")?;

    // Parse the AI's response (which should be JSON) into our CommitMessage struct
    let commit_message: CommitMessage = serde_json::from_str(&content).map_err(|e| {
        format!(
            "Failed to parse AI response as JSON: {}. Response was: {}",
            e, content
        )
    })?;

    Ok(commit_message)
}
