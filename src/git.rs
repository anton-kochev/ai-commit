use git2::Repository;

/// Re-export get_staged_diff from the diff module
pub use crate::diff::get_staged_diff;

/// Commit staged changes with the given commit message
pub fn commit_changes(commit_message: &str) -> Result<(), git2::Error> {
    // Discover and open the repository from the current directory
    let repo = Repository::discover(".")?;

    // Ensure there is something to commit
    let mut index = repo.index()?;
    if index.is_empty() {
        return Err(git2::Error::from_str("No staged changes to commit"));
    }

    // Write the index to a tree
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    // Create commit signature (user name, email, current time) from git config
    let signature = repo.signature()?;

    // Try to commit with HEAD as parent first
    let commit_result = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        commit_message,
        &tree,
        &[], // Start with no parents
    );

    // If that fails with "current tip is not the first parent", try to get the current HEAD
    // and use it as a parent
    match commit_result {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.message().contains("current tip is not the first parent") {
                eprintln!("Warning: HEAD has changed, attempting to get current HEAD");

                // Try to get the current HEAD commit
                if let Ok(head) = repo.head() {
                    if let Ok(head_commit) = head.peel_to_commit() {
                        // Try again with the current HEAD as parent
                        repo.commit(
                            Some("HEAD"),
                            &signature,
                            &signature,
                            commit_message,
                            &tree,
                            &[&head_commit],
                        )?;
                        return Ok(());
                    }
                }

                // If all else fails, try to commit without parents
                eprintln!(
                    "Warning: Could not get HEAD commit, attempting to commit without parents"
                );
                repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    commit_message,
                    &tree,
                    &[],
                )?;
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
