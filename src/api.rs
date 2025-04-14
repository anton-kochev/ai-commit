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
        r#"You are an assistant that writes structured Git commit messages based on code diffs.

        Analyze the following code diff and return a JSON object in this format:

        {{
            "summary": "short, meaningful summary (one line)",
            "description": "brief but detailed explanation of important changes (2–4 lines max)"
        }}

        Guidelines:
        - Only describe changes that matter (skip trivial or cosmetic edits)
        - Focus on intent and effect of the change
        - Group related edits together
        - Avoid file names or line numbers unless critical
        - Do NOT include unnecessary details or noise
        - Return valid JSON only

        Code diff:
        {}
        "#,
        diff
    );

    // Estimate cost before proceeding
    // Model	Context Limit	Cost (per 1K tokens)	Notes
    // gpt-3.5-turbo	4K tokens	$0.0005 / $0.0015	Best cheap option
    // gpt-3.5-turbo-16k	16K tokens	$0.0005 / $0.0015	For longer diffs
    // gpt-4-turbo	128K tokens	$0.01 / $0.03	For premium needs only
    let model = "gpt-4.1-mini"; // Default model
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
