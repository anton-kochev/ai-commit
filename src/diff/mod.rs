use git2::{DiffFormat, Repository};
use log::{debug, warn};
use std::path::Path;

/// Get the staged diff with specified context lines
pub fn get_staged_diff(context_lines: u32) -> Result<String, git2::Error> {
    // Discover and open the repository from the current directory.
    // This searches upward from the current directory to find the .git directory,
    // allowing the command to work from any subdirectory within the repository.
    let repo = Repository::discover(".")?;

    // Get the repository working directory.
    let repo_path = repo.workdir().unwrap_or(Path::new("."));

    // Load ignore patterns from the .gitignore file.
    let ignore_set = match crate::ignore::load_ignore_patterns(repo_path) {
        Ok(set) => {
            debug!("Loaded ignore patterns: {}", set.len());
            set
        }
        Err(e) => {
            warn!("Warning: could not load ignore patterns: {}", e);
            // Fallback to an empty GlobSet.
            let builder = globset::GlobSetBuilder::new();
            builder.build().unwrap_or_else(|_| {
                warn!("Failed to create GlobSet, using empty set");
                Default::default()
            })
        }
    };

    // Get HEAD tree, if available.
    let head_tree = match repo.head() {
        Ok(reference) => Some(reference.peel_to_tree()?),
        Err(_) => None,
    };

    // Get the index (staged changes).
    let index = repo.index()?;

    // Create diff options with configurable context lines
    let mut diff_options = git2::DiffOptions::new();
    diff_options.context_lines(context_lines);

    // Create a diff between the HEAD tree (if any) and the current index.
    let diff = repo.diff_tree_to_index(head_tree.as_ref(), Some(&index), Some(&mut diff_options))?;

    // Generate the diff output
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
            // Add the line origin character (+, -, or space) before the content
            let origin = line.origin();
            match origin {
                '+' | '-' | ' ' => diff_str.push(origin),
                _ => {}
            }
            diff_str.push_str(content);
        }
        true
    })?;

    Ok(diff_str)
}
