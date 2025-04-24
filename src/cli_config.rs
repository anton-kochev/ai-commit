use clap::Parser;

use crate::api::provider::Provider;

/// Command-line arguments for ai-commit
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    /// Specify the model to use for generating the commit message.
    /// The value is cached in the config file, however,
    /// it can be overridden by using the `--model` again.
    #[arg(short = 'm', long)]
    pub model: Option<String>,

    /// Specify the API key provider and key in the form <provider>=<key>.
    #[arg(short = 'k', long = "api-key", value_name = "provider=key", value_parser = provider_key_parser)]
    pub api_key: Option<(String, String)>,
}

fn provider_key_parser(provider_key_string: &str) -> Result<(String, String), String> {
    // Expect exactly "<provider>=<key>"
    let (provider, api_key) = provider_key_string
        .split_once('=')
        .ok_or_else(|| String::from("must be in form <provider>=<key>"))?;

    // Validate the provider
    match Provider::validate(str::trim(provider)) {
        Ok(..) => {}
        Err(e) => {
            return Err(e.to_string());
        }
    }

    Ok((provider.to_string(), api_key.to_string()))
}
