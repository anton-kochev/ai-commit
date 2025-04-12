use dotenv::dotenv;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::error::Error;

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
/// Returns a CommitMessage struct containing the summary and description.
pub fn generate_commit_message(diff: &str) -> Result<CommitMessage, Box<dyn Error>> {
    // Load environment variables from a .env file if present.
    dotenv().ok();

    // Retrieve the API key from the environment.
    let api_key = env::var("OPENAI_API_KEY")?;
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

    // Build the JSON request body.
    let request_body = json!({
       "model": "gpt-4",
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
    let commit_message: CommitMessage = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse AI response as JSON: {}. Response was: {}", e, content))?;

    Ok(commit_message)
}
