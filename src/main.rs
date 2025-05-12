use api::provider::Provider;
use clap::Parser;
use env_logger::Builder;
use log::{error, info, trace};

mod api;
mod cli;
mod cli_config;
mod config_manager;
mod cost_estimation;
mod git;
mod ignore;
mod prompt;

use cli::UserChoice;
use cli_config::CliConfig;

fn main() {
    // Parse the command-line arguments
    let cli_config = CliConfig::parse();

    // Initialize the logger with a default level
    // This will use RUST_LOG if set, otherwise fall back to 'info' level
    Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    trace!("Starting ai-commit!");
    trace!("Parsed cli args: {:?}", cli_config);

    // Load existing configuration or use defaults
    let config = match config_manager::load_config(cli_config) {
        Ok(config) => config,
        Err(..) => {
            return;
        }
    };

    info!("Using model: {}", config.get_model());

    // Retrieve the staged diff
    let diff = match git::get_staged_diff() {
        Ok(diff) => {
            if diff.is_empty() {
                info!("No staged changes found. Nothing to commit.");
                return;
            }
            diff
        }
        Err(e) => {
            error!("Failed to get staged diff: {}", e);
            return;
        }
    };

    // Get the prompt for the model input
    let prompt = format!("{}\n\nDiff:\n{}", prompt::get_instructions(), &diff);
    // Estimate cost before proceeding
    let cost = cost_estimation::estimate_cost(config.get_model(), &prompt);

    info!("{}", cost_estimation::format_cost_estimate(&cost));

    // Generate the initial commit message suggestion
    if !cli::prompt_for_confirmation("Do you want to proceed?") {
        info!("User canceled the operation.");
        return;
    }

    let (provider, key) = config.get_provider_key();
    let api = Provider::create_provider(provider, key).expect("Failed to create provider");

    let commit_message = match api.generate_commit_message(config.get_model(), &diff) {
        Ok(msg) => format!(
            "{}{}",
            msg.summary,
            match msg.description {
                Some(desc) => format!("\n\n{}", desc),
                None => "".to_string(),
            }
        ),
        Err(e) => {
            error!("Failed to generate commit message: {}", e);
            return;
        }
    };

    println!("{}", &commit_message);

    match cli::prompt_user_for_action() {
        UserChoice::Commit => {
            info!("User accepted the commit message.");

            // Commit the changes with the accepted message
            if let Err(e) = git::commit_changes(commit_message) {
                error!("Failed to commit changes: {}", e);
                return;
            }
        }
        UserChoice::Cancel => {
            info!("User canceled the commit.");
            return;
        }
    }

    info!("Successfully committed changes");
}
