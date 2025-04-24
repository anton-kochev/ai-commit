use log::trace;
use reqwest::{blocking::Client, header::CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

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

pub struct OpenAiApi {
    api_key: String,
    api_url: String,
    client: reqwest::blocking::Client,
}

impl OpenAiApi {
    pub fn new(api_key: String) -> Self {
        OpenAiApi {
            api_key,
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    /// Generates a commit message by sending the provided diff to the OpenAI ChatGPT API.
    /// Returns the commit message as a string.
    pub fn generate_commit_message(
        self,
        model: &str,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Create a client with increased timeout
        trace!("Creating HTTP client with 120 seconds timeout");

        // Build the JSON request body.
        let request_body = json!({
           "model": model,
           "messages": [
                { "role": "user", "content": prompt }
           ]
        });

        trace!("Request body");
        trace!("{}", serde_json::to_string_pretty(&request_body)?);

        // Send the POST request to the OpenAI Chat Completions API.
        let response = self
            .client
            .post(self.api_url)
            .bearer_auth(self.api_key)
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
}
