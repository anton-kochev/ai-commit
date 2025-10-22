use std::fmt;

use reqwest::StatusCode;

use super::openai::OpenAiApi;

pub type ProviderResult<T> = std::result::Result<T, ProviderError>;

pub struct CommitMessage {
    pub description: Option<String>,
    pub summary: String,
    pub warning: Option<String>,
}

#[derive(Debug)]
pub enum ProviderError {
    ApiError(StatusCode, String),
    UnsupportedProvider(String),
    InvalidFormat,
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProviderError::ApiError(status, msg) => format!("API Error ({}): {}", status, msg),
                ProviderError::UnsupportedProvider(provider) =>
                    format!("Unsupported provider: {}", provider),
                ProviderError::InvalidFormat => "Invalid format".to_string(),
            }
        )
    }
}

pub enum Provider {
    OpenAI(OpenAiApi),
    // Anthropic(),
    // GoogleGemini(),
}

impl Provider {
    pub fn validate(s: &str) -> ProviderResult<()> {
        match s {
            "openai" => Ok(()),
            // "anthropic" => Ok(()),
            // "google-gemini" => Ok(()),
            p => Err(ProviderError::UnsupportedProvider(p.to_string())),
        }
    }

    pub fn create_provider(provider: &str, api_key: &str) -> ProviderResult<Self> {
        match provider {
            "openai" => Ok(Provider::OpenAI(OpenAiApi::new(api_key.to_string())?)),
            p => Err(ProviderError::UnsupportedProvider(p.to_string())),
        }
    }

    pub fn generate_commit_message(
        self,
        model: &str,
        prompt: &str,
        context: Option<&str>,
    ) -> ProviderResult<CommitMessage> {
        match self {
            Provider::OpenAI(api) => api.generate_commit_message(model, prompt, context),
        }
    }
}
