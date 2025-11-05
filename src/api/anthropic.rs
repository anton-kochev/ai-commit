use log::{error, trace};
use reqwest::{blocking::Client, header::CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;

use super::provider::{CommitMessage, ProviderResult};
use crate::prompt;

/// Structs for deserializing the Anthropic Messages response.
#[derive(Deserialize)]
struct MessageResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ContentBlock {
    ToolUse {
        #[allow(dead_code)]
        id: String,
        #[allow(dead_code)]
        name: String,
        input: serde_json::Value,
    },
    Text {
        #[allow(dead_code)]
        text: String,
    },
}

#[derive(Deserialize)]
struct CommitMessageToolInput {
    description: Option<String>,
    summary: String,
    warning: Option<String>,
}

/// Struct for the Anthropic API client.
pub struct AnthropicApi {
    api_key: String,
    api_url: String,
    client: reqwest::blocking::Client,
}

impl AnthropicApi {
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

        Ok(AnthropicApi {
            api_key,
            api_url: "https://api.anthropic.com/v1/messages".to_string(),
            client,
        })
    }

    /// Generates a commit message by sending the provided diff to the Anthropic Messages API.
    pub fn generate_commit_message(
        self,
        model: &str,
        diff: &str,
        context: Option<&str>,
    ) -> ProviderResult<CommitMessage> {
        trace!("Creating HTTP client with 120 seconds timeout");

        // Build the JSON request body.
        let user_description = context.unwrap_or("");
        let content = format!(
            "Git Diff:\n{}\n\nUser Description:\n{}",
            diff, user_description
        );

        let request_body = json!({
            "model": model,
            "max_tokens": 1024,
            "tools": [
                {
                    "name": "git_commit_message",
                    "description": "Generate a commit message from a diff",
                    "input_schema": {
                        "type": "object",
                        "properties": {
                            "description": {
                                "type": "string",
                                "description": "A detailed description of the changes"
                            },
                            "summary": {
                                "type": "string",
                                "description": "A one-sentence description of the key change, starting with a capital letter."
                            },
                            "warning": {
                                "type": "string",
                                "description": "A string containing all detected potential sensitive information, or null if none found."
                            }
                        },
                        "required": ["summary"]
                    }
                }
            ],
            "tool_choice": {
                "type": "tool",
                "name": "git_commit_message"
            },
            "messages": [
                {
                    "role": "user",
                    "content": format!("{}\n\n{}", prompt::get_system_prompt(), content)
                }
            ]
        });

        // Send the POST request to the Anthropic Messages API.
        let response = self
            .client
            .post(self.api_url)
            .header("x-api-key", self.api_key)
            .header("anthropic-version", "2023-06-01")
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
        let json_response: MessageResponse = response.json().map_err(|e| {
            error!("Failed to parse JSON response: {}", e);
            crate::api::provider::ProviderError::InvalidFormat
        })?;

        // Extract the commit message content from the response.
        let tool_use = json_response
            .content
            .into_iter()
            .find_map(|block| match block {
                ContentBlock::ToolUse { input, .. } => Some(input),
                _ => None,
            })
            .ok_or_else(|| {
                error!("No tool_use block found in the response");
                crate::api::provider::ProviderError::InvalidFormat
            })?;

        let commit_message = serde_json::from_value::<CommitMessageToolInput>(tool_use)
            .map_err(|e| {
                error!("Failed to parse tool input: {}", e);
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
