use clap::Parser;
use cost_estimation::print_cost;
use dotenv;
use env_logger::Builder;
use log::{error, info, trace};

mod api;
mod cli;
mod cost_estimation;
mod git;
mod ignore;
mod prompt;

use cli::UserChoice;

/// Command-line arguments for ai-commit
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run the program to estimate the cost of the commit message generation,
    /// without actually sending any requests and committing the changes.
    #[arg(long)]
    estimate_only: bool,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize the logger with a default level
    // This will use RUST_LOG if set, otherwise fall back to info level
    Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    trace!("Starting ai-commit!");
    trace!("Parsed arguments: {:?}", args);

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Define the model
    let model = "o1";

    info!("Using model: {}", &model);

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
    let prompt = prompt::get_prompt(diff);
    // Estimate cost before proceeding
    let cost_estimate = cost_estimation::estimate_cost(&model, &prompt);

    print_cost(&cost_estimate);

    // If estimate_only is true, exit after printing the cost
    if args.estimate_only {
        return;
    }

    // Generate the initial commit message suggestion
    let commit_message = match api::generate_commit_message(&model, prompt) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Failed to generate commit message: {}", e);
            return;
        }
    };

    // Enter an interactive loop for user decision
    loop {
        match cli::prompt_user_for_action(&commit_message) {
            UserChoice::Commit => {
                info!("User accepted the commit message.");

                // Commit the changes with the accepted message
                if let Err(e) = git::commit_changes(commit_message) {
                    error!("Failed to commit changes: {}", e);
                    return;
                }
                break;
            }
            UserChoice::Cancel => {
                info!("User canceled the commit.");
                return;
            }
        }
    }

    info!("Successfully committed changes");
}
