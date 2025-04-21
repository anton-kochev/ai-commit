use directories::ProjectDirs;
use log::{error, trace, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, ErrorKind};
use std::path::PathBuf;

use crate::cli_config::CliConfig;

const QUALIFIER: &str = "dev";
const ORGANIZATION: &str = "anton-kochev";
const APPLICATION: &str = "ai-commit";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub api_key: Option<String>,
    pub api_provider: Option<String>,
    pub model: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            api_key: None,
            api_provider: None,
            model: None,
        }
    }
}

impl AppConfig {
    pub fn get_model(&self) -> &str {
        self.model.as_deref().expect("Model field is missing")
    }

    pub fn get_api_key(&self) -> &str {
        self.api_key.as_deref().expect("ApiKey field is missing")
    }

    pub fn get_api_provider(&self) -> &str {
        self.api_provider
            .as_deref()
            .expect("ApiProvider field is missing")
    }
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
        .map(|proj_dirs| proj_dirs.config_dir().join(CONFIG_FILE_NAME))
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
        config.api_key = Some(key);
        config.api_provider = Some(provider);
    }
    if let Some(model) = cli_config.model {
        config.model = Some(model);
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
