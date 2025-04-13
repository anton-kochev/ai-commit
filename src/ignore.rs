use globset::{Glob, GlobSet, GlobSetBuilder};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Loads ignore patterns from a .ai-commit-ignore file located at the repository root.
/// If the file does not exist, an empty GlobSet is returned (meaning no exclusions).
pub fn load_ignore_patterns(repo_path: &Path) -> Result<GlobSet, Box<dyn Error>> {
    let ignore_path = repo_path.join(".ai-commit-ignore");
    let mut builder = GlobSetBuilder::new();

    if ignore_path.exists() {
        let file = File::open(ignore_path)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            // Skip empty lines and comments.
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // For simplicity, skip negation patterns (lines starting with '!').
            if trimmed.starts_with('!') {
                continue;
            }
            // Create and add the glob
            let glob = Glob::new(trimmed)?;
            builder.add(glob);
        }
    }
    let globset = builder.build()?;
    Ok(globset)
}
