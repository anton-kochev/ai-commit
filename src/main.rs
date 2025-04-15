use clap::Parser;
use cost_estimation::print_cost;
use dotenv;
use env_logger::Builder;
use log::{error, info, trace, LevelFilter};

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
    /// Run in dry-run mode (no API calls will be made)
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize the logger with a default level
    // This will use RUST_LOG if set, otherwise fall back to info level
    Builder::new()
        .filter_level(LevelFilter::Info) // Default level if RUST_LOG is not set
        .init();

    trace!("Starting ai-commit!");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Define the model
    let model = "o1";

    info!("Using model: {}", &model);

    // Load ignore patterns from the repository's .gitignore file
    match ignore::load_ignore_patterns(std::env::current_dir().unwrap().as_path()) {
        Ok(set) => trace!("Loaded {} ignore patterns", set.len()),
        Err(e) => error!("Error loading ignore patterns: {}", e),
    }

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
    // Check if we should run in dry-run mode
    if args.dry_run {
        info!("Dry-run mode");
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
