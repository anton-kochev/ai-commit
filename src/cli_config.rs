use anyhow::{Context, Result};
use clap::Parser;

use crate::api::provider::Provider;

/// Command-line arguments for ai-commit
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliConfig {
    /// Specify the API key provider and key in the form <provider>=<key>.
    #[arg(short = 'k', long = "api-key", value_name = "provider=key", value_parser = provider_key_parser)]
    pub api_key: Option<(String, String)>,
    /// Specify the model to use for generating the commit message.
    /// The value is cached in the config file, however,
    /// it can be overridden by using the `--model` again.
    #[arg(short = 'm', long)]
    pub model: Option<String>,
}

fn provider_key_parser(provider_key_string: &str) -> Result<(String, String)> {
    // Expect exactly "<provider>=<key>"
    let (provider, api_key) = provider_key_string
        .split_once('=')
        .context("Must be in form <provider>=<key>")?;

    // Validate the provider
    if let Err(e) = Provider::validate(str::trim(provider)) {
        return Err(anyhow::anyhow!(e));
    }

    Ok((provider.to_string(), api_key.to_string()))
}
