use std::process;

use api::provider::Provider;
use clap::Parser;
use commit_editor::edit_message;
use dialoguer::console::{self, Style};
use env_logger::Builder;
use log::{error, info, trace};

mod api;
mod cli;
mod cli_config;
mod commit_editor;
mod config_manager;
mod cost_estimation;
mod git;
mod ignore;
mod prompt;

use cli::UserChoice;
use cli_config::CliConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let cli_config = CliConfig::parse();
    let terminal = console::Term::stdout();

    // Initialize the logger with a default level
    // This will use RUST_LOG if set, otherwise fall back to 'info' level
    Builder::from_env(env_logger::Env::default()).init();

    trace!("Starting ai-commit!");
    trace!("Parsed cli args: {:?}", &cli_config);

    // Load existing configuration or use defaults
    let config = match config_manager::load_config(cli_config) {
        Ok(config) => config,
        Err(..) => {
            error!("Failed to load configuration. Please check your settings.");
            terminal.write_line("Error loading configuration")?;
            process::exit(1);
        }
    };

    terminal.write_line(&format!("Using model: {}", config.get_model()))?;

    // Retrieve the staged diff
    let diff = match git::get_staged_diff() {
        Ok(diff) => {
            if diff.is_empty() {
                terminal.write_line("No staged changes found")?;
                process::exit(0);
            }
            diff
        }
        Err(e) => {
            error!("{}", e);
            terminal.write_line("Error retrieving staged changes")?;
            process::exit(1);
        }
    };

    // Get the prompt for the model input
    let prompt = format!("{}\n\nDiff:\n{}", prompt::get_system_prompt(), &diff);
    // Estimate cost before proceeding
    let cost = cost_estimation::estimate_cost(config.get_model(), &prompt)?;

    println!("{}", cost_estimation::format_cost_estimate(&cost));

    // Generate the initial commit message suggestion
    if !cli::prompt_for_confirmation("Do you want to proceed?") {
        terminal.write_line("Operation canceled by the user")?;
        process::exit(0);
    }

    let (provider, key) = config.get_provider_key();
    let api = Provider::create_provider(provider, key).expect("Failed to create provider");

    terminal.write_line("Generating commit message...")?;

    let (mut commit_message, warning_message) =
        match api.generate_commit_message(config.get_model(), &diff, config.get_user_desc()) {
            Ok(msg) => (
                format!(
                    "{}{}",
                    msg.summary,
                    match msg.description {
                        Some(desc) => format!("\n\n{}", desc),
                        None => "".to_string(),
                    }
                ),
                msg.warning,
            ),
            Err(e) => {
                error!("Failed to generate commit message: {}", e);
                terminal.write_line("Error generating commit message")?;
                process::exit(1);
            }
        };

    terminal.write_line(&commit_message)?;

    if let Some(warning) = warning_message {
        let warning_style = Style::new().white().bold().on_red();
        terminal.write_line(&warning_style.apply_to(warning).to_string())?;
    }

    handle_commit_message(&mut commit_message)?;

    terminal.write_line("Changes commited successfully")?;

    Ok(())
}

fn handle_commit_message(commit_message: &mut String) -> Result<(), std::io::Error> {
    match cli::prompt_user_for_action() {
        UserChoice::Edit => {
            info!("User chose to edit the commit message.");
            edit_message(commit_message)?;

            if let Err(e) = git::commit_changes(commit_message) {
                error!("Failed to commit changes: {}", e);
            }

            Ok(())
        }
        UserChoice::Commit => {
            info!("User accepted the commit message.");

            if let Err(e) = git::commit_changes(commit_message) {
                error!("Failed to commit changes: {}", e);
            }

            Ok(())
        }
        _ => {
            info!("User canceled the commit.");
            Ok(())
        }
    }
}
