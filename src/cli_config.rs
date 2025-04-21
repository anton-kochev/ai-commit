use clap::Parser;

/// Represents the supported models for the API.
enum Model {
    Gpt3_5Turbo,
    Gpt4,
    Gpt4_1,
    Gpt4_1Mini,
    Gpt4_1Nano,
    Gpt4_5Preview,
    Gpt4o,
    Gpt4oMini,
    O1,
    O3,
    O1Mini,
    O1Pro,
    O3Mini,
}

impl Model {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "gpt-3.5-turbo" => Ok(Model::Gpt3_5Turbo),
            "gpt-4" => Ok(Model::Gpt4),
            "gpt-4.1" => Ok(Model::Gpt4_1),
            "gpt-4.1-mini" => Ok(Model::Gpt4_1Mini),
            "gpt-4.1-nano" => Ok(Model::Gpt4_1Nano),
            "gpt-4.5-preview" => Ok(Model::Gpt4_5Preview),
            "gpt-4o" => Ok(Model::Gpt4o),
            "gpt-4o-mini" => Ok(Model::Gpt4oMini),
            "o1" => Ok(Model::O1),
            "o1-mini" => Ok(Model::O1Mini),
            "o1-pro" => Ok(Model::O1Pro),
            "o3" => Ok(Model::O3),
            "o3-mini" => Ok(Model::O3Mini),
            _ => Err(format!("Unsupported model: {}", s)),
        }
    }
}

/// Represents the supported providers for the API keys.
enum Provider {
    OpenAI,
    Anthropic,
}

impl Provider {
    /// Converts a string to a Providers enum variant.
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "openai" => Ok(Provider::OpenAI),
            "anthropic" => Ok(Provider::Anthropic),
            _ => Err(format!("Unsupported provider: {}", s)),
        }
    }
}

/// Command-line arguments for ai-commit
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    /// Specify the model to use for generating the commit message.
    /// The value is cached in the config file, however,
    /// it can be overridden by using the `--model` again.
    #[arg(short = 'm', long, value_parser = model_parser)]
    pub model: Option<String>,

    /// Specify the API key provider and key in the form <provider>=<key>.
    #[arg(short = 'k', long = "api-key", value_name = "provider=key", value_parser = provider_key_parser)]
    pub api_key: Option<(String, String)>,
}

fn model_parser(model_string: &str) -> Result<String, String> {
    // Validate the model
    Model::from_str(model_string)?;

    Ok(model_string.to_string())
}

fn provider_key_parser(provider_key_string: &str) -> Result<(String, String), String> {
    // Expect exactly "<provider>=<key>"
    let (provider, api_key) = provider_key_string
        .split_once('=')
        .ok_or_else(|| String::from("must be in form <provider>=<key>"))?;

    // Validate the provider
    Provider::from_str(provider)?;

    Ok((provider.to_string(), api_key.to_string()))
}
