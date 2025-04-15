use git2::{DiffFormat, Repository};
use std::path::Path;

pub fn get_staged_diff() -> Result<String, git2::Error> {
    // Open the current repository.
    let repo = Repository::open(".")?;

    // Get the repository working directory.
    let repo_path = repo.workdir().unwrap_or(Path::new("."));

    // Load ignore patterns from the .gitignore file.
    let ignore_set = match crate::ignore::load_ignore_patterns(repo_path) {
        Ok(set) => set,
        Err(e) => {
            eprintln!("Warning: could not load ignore patterns: {}", e);
            // Fallback to an empty GlobSet.
            let builder = globset::GlobSetBuilder::new();
            builder.build().unwrap()
        }
    };

    // Get HEAD tree, if available.
    let head_tree = match repo.head() {
        Ok(reference) => Some(reference.peel_to_tree()?),
        Err(_) => None,
    };

    // Get the index (staged changes).
    let index = repo.index()?;

    // Create a diff between the HEAD tree (if any) and the current index.
    let diff = repo.diff_tree_to_index(head_tree.as_ref(), Some(&index), None)?;

    // Prepare a String buffer to collect diff output.
    let mut diff_str = String::new();

    // Print the diff in Patch format, filtering out ignored files.
    diff.print(DiffFormat::Patch, |delta, _hunk, line| {
        // Get the file path from the diff delta (prefer new file path, fallback to old).
        let file_path = delta.new_file().path().or(delta.old_file().path());
        if let Some(path) = file_path {
            if ignore_set.is_match(path) {
                // Skip lines for files that match ignore patterns.
                return true;
            }
        }
        if let Ok(content) = std::str::from_utf8(line.content()) {
            diff_str.push_str(content);
        }
        true
    })?;

    Ok(diff_str)
}

/// Commit staged changes with the given commit message
pub fn commit_changes(commit_message: String) -> Result<(), git2::Error> {
    // Open the current repository
    let repo = Repository::open(".")?;

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
        &commit_message,
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
                            &commit_message,
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
                    &commit_message,
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
