use dotenv;
use env_logger::Builder;
use log::{error, info, trace, LevelFilter};

mod api;
mod cli;
mod cost;
mod git;
mod ignore;

use cli::UserChoice;

fn main() {
    // Initialize the logger with a default level
    // This will use RUST_LOG if set, otherwise fall back to info level
    Builder::new()
        .filter_level(LevelFilter::Info) // Default level if RUST_LOG is not set
        .init();

    trace!("Starting ai-commit!");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Check if we should run in dry-run mode
    let dry_run = std::env::var("DRY_RUN").is_ok();
    if dry_run {
        info!("Running in dry-run mode (no API calls will be made)");
    }

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

    // Generate the initial commit message suggestion
    let mut commit_message = match api::generate_commit_message(&diff, dry_run) {
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
                if let Err(e) = git::commit_changes(&commit_message) {
                    error!("Failed to commit changes: {}", e);
                    return;
                }
                break;
            }
            UserChoice::Regenerate => {
                info!("User chose to regenerate the commit message.");
                match api::generate_commit_message(&diff, dry_run) {
                    Ok(new_msg) => {
                        commit_message = new_msg;
                    }
                    Err(e) => {
                        error!("Failed to regenerate commit message: {}", e);
                        return;
                    }
                }
            }
            UserChoice::Cancel => {
                info!("User canceled the commit.");
                return;
            }
        }
    }

    info!("Successfully committed changes");
}
