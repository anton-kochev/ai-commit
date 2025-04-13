use env_logger::Builder;
use log::{error, info, trace, LevelFilter};

mod api;
mod cli;
mod git;
mod ignore;

fn main() {
    // Initialize the logger with a default level
    // This will use RUST_LOG if set, otherwise fall back to info level
    Builder::new()
        .filter_level(LevelFilter::Info) // Default level if RUST_LOG is not set
        .init();

    trace!("Starting ai-commit");

    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Check if we should run in dry-run mode
    let dry_run = std::env::var("DRY_RUN").is_ok();
    if dry_run {
        info!("Running in dry-run mode - API calls will be traced but not executed");
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
                info!("No staged changes found.");
                return;
            }
            diff
        }
        Err(e) => {
            error!("Error retrieving staged diff: {}", e);
            return;
        }
    };

    // Generate the initial commit message suggestion
    let mut commit_msg = match api::generate_commit_message(&diff, dry_run) {
        Ok(msg) => msg,
        Err(e) => {
            error!("Error generating commit message: {}", e);
            return;
        }
    };

    // Enter an interactive loop for user decision
    loop {
        match cli::prompt_user_for_action(&commit_msg) {
            cli::UserChoice::Commit => {
                info!("User accepted the commit message.");
                // Commit the changes with the accepted message
                match git::commit_changes(&commit_msg.summary, &commit_msg.description) {
                    Ok(_) => {
                        info!("Changes committed successfully with message:");
                        info!("Summary: {}", commit_msg.summary);
                        if !commit_msg.description.is_empty() {
                            info!("Description: {}", commit_msg.description);
                        }
                    }
                    Err(e) => {
                        error!("Failed to commit changes: {}", e);
                    }
                }
                break;
            }
            cli::UserChoice::Regenerate => {
                info!("User chose to regenerate the commit message.");
                match api::generate_commit_message(&diff, dry_run) {
                    Ok(new_msg) => commit_msg = new_msg,
                    Err(e) => {
                        error!("Error regenerating commit message: {}", e);
                        return;
                    }
                }
            }
            cli::UserChoice::Cancel => {
                info!("User canceled the commit.");
                return;
            }
        }
    }

    info!("ai-commit completed successfully");
}
