use log::trace;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use serde_json::json;
use std::env;

use crate::config_manager::AppConfig;

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

/// Generates a commit message by sending the provided diff to the OpenAI ChatGPT API.
/// If dry_run is true, it will trace the request instead of making an actual API call.
/// Returns the commit message as a string.
pub fn generate_commit_message(
    config: &AppConfig,
    prompt: String,
) -> Result<String, Box<dyn std::error::Error>> {
    if !prompt_for_confirmation() {
        return Err("Operation canceled by user.".into());
    }

    // Create a client with increased timeout
    trace!("Creating HTTP client with 120 seconds timeout");

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(120)) // Increase timeout to 60 seconds
        .build()?;

    // Build the JSON request body.
    let request_body = json!({
       "model": config.get_model(),
       "messages": [
            { "role": "user", "content": prompt }
       ]
    });

    trace!("Request body");
    trace!("{}", serde_json::to_string_pretty(&request_body)?);

    // Send the POST request to the OpenAI Chat Completions API.
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(config.get_api_key())
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

    // Return the trimmed content
    Ok(content.trim().to_string())
}

/// Prompts the user for confirmation based on the token count and estimated cost
pub fn prompt_for_confirmation() -> bool {
    let skip_confirmation = env::var("AI_COMMIT_SKIP_COST_CONFIRM").is_ok();
    if skip_confirmation {
        trace!(
            "Skipping cost confirmation due to AI_COMMIT_SKIP_COST_CONFIRM environment variable"
        );
        return true;
    }

    // Use dialoguer to prompt the user.
    // Return the user's choice.
    dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Do you want to proceed?")
        .default(false)
        .interact()
        .unwrap_or(false)
}
