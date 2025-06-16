use log::{error, trace};
use reqwest::{blocking::Client, header::CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

use super::provider::{CommitMessage, ProviderResult};
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
    description: Option<String>,
    summary: String,
    warning: Option<String>,
}

/// Struct for the OpenAI API client.
pub struct OpenAiApi {
    api_key: String,
    api_url: String,
    client: reqwest::blocking::Client,
}

impl OpenAiApi {
    pub fn new(api_key: String) -> ProviderResult<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .map_err(|e| {
                error!("Failed to create HTTP client: {}", e);
                crate::api::provider::ProviderError::ApiError(
                    reqwest::StatusCode::INTERNAL_SERVER_ERROR,
                    e.to_string(),
                )
            })?;

        Ok(OpenAiApi {
            api_key,
            api_url: "https://api.openai.com/v1/chat/completions".to_string(),
            client,
        })
    }

    /// Generates a commit message by sending the provided diff to the OpenAI ChatGPT API.
    pub fn generate_commit_message(self, model: &str, diff: &str) -> ProviderResult<CommitMessage> {
        trace!("Creating HTTP client with 120 seconds timeout");

        // Build the JSON request body.
        let request_body = json!({
           "model": model,
           "messages": [
            {
                "role": "system",
                "content": prompt::get_system_prompt()
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
                            "description": { "type": "string" },
                            "summary": { "type": "string",
                                "description": "A one-sentence description of the key change, starting with a capital letter."
                            },
                            "warning": {
                                "type": "string",
                                "description": "A string containing all detected potential sensitive information, or `null` if none found.",
                            }
                        },
                        "required": ["summary"],
                        "additionalProperties": false
                    }
                }
            ],
            "function_call": { "name": "git_commit_message" }
        });

        // Send the POST request to the OpenAI Chat Completions API.
        let response = self
            .client
            .post(self.api_url)
            .bearer_auth(self.api_key)
            .header(CONTENT_TYPE, "application/json")
            .json(&request_body)
            .send()
            .map_err(|e| {
                error!("Failed to send request: {}", e);
                crate::api::provider::ProviderError::ApiError(
                    e.status()
                        .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
                    e.to_string(),
                )
            })?;

        // Deserialize the JSON response.
        let json_response: ChatResponse = response.json().map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            crate::api::provider::ProviderError::InvalidFormat
        })?;

        // Extract the commit message content from the response.
        let function_call_result = &json_response
            .choices
            .first()
            .ok_or_else(|| {
                error!("No choices found in the response");
                crate::api::provider::ProviderError::InvalidFormat
            })?
            .message
            .function_call
            .arguments;

        let commit_message = serde_json::from_str::<ChatFunctionCallResult>(function_call_result)
            .map_err(|e| {
            error!("Failed to parse function call arguments: {}", e);
            crate::api::provider::ProviderError::InvalidFormat
        })?;

        // Return the CommitMessage
        Ok(CommitMessage {
            description: commit_message.description,
            summary: commit_message.summary,
            warning: commit_message.warning,
        })
    }
}
