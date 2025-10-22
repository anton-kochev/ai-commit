use directories::ProjectDirs;
use log::{error, trace, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

use crate::cli_config::CliConfig;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct AppConfig {
    pub api_key: Option<String>,
    pub api_provider: Option<String>,
    pub model: Option<String>,
    pub user_desc: Option<String>,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            api_key: None,
            api_provider: None,
            model: None,
            user_desc: None,
        }
    }

    pub fn api_key(&mut self, value: String) -> &mut Self {
        self.api_key = Some(value);
        self
    }

    pub fn api_provider(&mut self, value: String) -> &mut Self {
        self.api_provider = Some(value);
        self
    }

    pub fn model(&mut self, value: String) -> &mut Self {
        self.model = Some(value);
        self
    }

    pub fn user_desc(&mut self, value: String) -> &mut Self {
        self.user_desc = Some(value);
        self
    }

    pub fn get_model(&self) -> &str {
        self.model.as_deref().expect("Model field is missing")
    }

    pub fn get_user_desc(&self) -> Option<&str> {
        self.user_desc.as_deref()
    }

    pub fn get_provider_key(&self) -> (&str, &str) {
        self.api_key
            .as_deref()
            .and_then(|key| self.api_provider.as_deref().map(|provider| (provider, key)))
            .expect("API key or provider field is missing")
    }
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("dev", "anton-kochev", "ai-commit")
        .map(|proj_dirs| proj_dirs.config_dir().join("config.json"))
}

pub fn load_config(cli_config: CliConfig) -> Result<AppConfig, &'static str> {
    let mut config = match get_config_path() {
        Some(config_path) => {
            trace!("Loading configuration from {}", config_path.display());

            match config_path.exists() {
                true => match fs::read_to_string(&config_path) {
                    Ok(content) => match serde_json::from_str(&content) {
                        Ok(config) => {
                            trace!("Configuration: {:?}", config);

                            config
                        }
                        Err(e) => {
                            warn!("Failed to parse config file. {}", e);

                            AppConfig::default()
                        }
                    },
                    Err(e) => {
                        warn!("Failed to read config file: {}.", e);

                        AppConfig::default()
                    }
                },
                _ => {
                    // Config file not found, return default config silently.
                    // No info! log here anymore.
                    trace!("Config file not found.");

                    AppConfig::default()
                }
            }
        }
        None => {
            warn!("Could not determine config directory.");

            AppConfig::default()
        }
    };

    // Update the config with CLI arguments
    if let Some((provider, key)) = cli_config.api_key {
        config.api_key(key).api_provider(provider);
    }
    if let Some(model) = cli_config.model {
        config.model(model);
    }
    if let Some(context) = cli_config.context {
        config.user_desc(context);
    }

    // Validate the mandatory fields
    if config.model.is_none() {
        error!("Model is not set. Please use -m/--model.");

        return Err("Model is not set.");
    }
    if config.api_key.is_none() || config.api_provider.is_none() {
        error!("API key is not set. Please use -k/--api-key.");

        return Err("API key is not set.");
    }

    // Save the updated config
    if let Err(e) = save_config(&config) {
        error!("Failed to save config: {}", e);

        return Err("Failed to save config.");
    }

    Ok(config)
}

fn save_config(config: &AppConfig) -> io::Result<()> {
    match get_config_path() {
        Some(config_path) => {
            // Ensure the parent directory exists
            if let Some(parent_dir) = config_path.parent() {
                fs::create_dir_all(parent_dir)?;
            }

            // Serialize and save the config
            let content = serde_json::to_string_pretty(&config)?;

            fs::write(&config_path, content)?;

            trace!("Config file successfully updated");

            Ok(())
        }
        None => Err(io::Error::new(
            ErrorKind::NotFound,
            "Could not determine config directory.",
        )),
    }
}
