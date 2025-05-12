use log::trace;
use reqwest::{blocking::Client, header::CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

use super::provider::CommitMessage;
use crate::prompt;

/// Structs for deserializing the OpenAI Chat Completions response.
/// These structs are used to parse the JSON response from the OpenAI API.
#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    function_call: ChatFunctionCall,
}

#[derive(Deserialize)]
struct ChatFunctionCall {
    arguments: String,
}

#[derive(Deserialize)]
struct ChatFunctionCallResult {
    summary: String,
    description: Option<String>,
}

/// Struct for the OpenAI API client.
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
    pub fn generate_commit_message(
        self,
        model: &str,
        diff: &str,
    ) -> Result<CommitMessage, Box<dyn std::error::Error>> {
        trace!("Creating HTTP client with 120 seconds timeout");

        // Build the JSON request body.
        let request_body = json!({
           "model": model,
           "messages": [
            {
                "role": "developer",
                "content": prompt::get_instructions()
            },
            {
                "role": "user",
                "content": diff
            }],
            "functions": [
                {
                    "name": "git_commit_message",
                    "description": "Generate a commit message from a diff",
                    "parameters": {
                        "type": "object",
                        "properties": {
                            "summary": { "type": "string" },
                            "description": { "type": "string" }
                        },
                        "required": ["summary"],
                        "additionalProperties": false
                    }
                }
            ],
            "function_call": { "name": "git_commit_message" }
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

        // Extract the commit message content from the response.
        let function_call_result = &json_response
            .choices
            .first()
            .ok_or("No choices returned from API")?
            .message
            .function_call
            .arguments;

        let commit_message = serde_json::from_str::<ChatFunctionCallResult>(function_call_result)?;

        // Return the CommitMessage
        Ok(CommitMessage {
            summary: commit_message.summary,
            description: commit_message.description,
        })
    }
}
