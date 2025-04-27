use super::openai::OpenAiApi;

pub struct CommitMessage {
    pub summary: String,
    pub description: Option<String>,
}

pub enum Provider {
    OpenAI(OpenAiApi),
    // Anthropic(),
    // GoogleGemini(),
}

impl Provider {
    pub fn validate(s: &str) -> Result<(), &'static str> {
        match s {
            "openai" => Ok(()),
            // "anthropic" => Ok(()),
            // "google-gemini" => Ok(()),
            _ => Err("Unsupported provider"),
        }
    }

    pub fn create_provider(provider: &str, api_key: &str) -> Result<Self, &'static str> {
        match provider {
            "openai" => Ok(Provider::OpenAI(OpenAiApi::new(api_key.to_string()))),
            _ => Err("Unsupported provider"),
        }
    }

    pub fn generate_commit_message(
        self,
        model: &str,
        prompt: &str,
    ) -> Result<CommitMessage, Box<dyn std::error::Error>> {
        match self {
            Provider::OpenAI(api) => api.generate_commit_message(model, prompt),
        }
    }
}
