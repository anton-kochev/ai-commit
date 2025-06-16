use std::io::Read;

use log::trace;

pub fn edit_message(commit_message: &mut String) -> Result<(), std::io::Error> {
    let mut path = std::env::temp_dir();
    path.push("ai-commit");
    std::fs::write(&path, &commit_message)?;

    // Determine the user's default editor
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    trace!("Using '{}' editor", editor);
    match std::process::Command::new(&editor).arg(&path).status() {
        Ok(status) => {
            commit_message.clear();
            if status.success() {
                let mut file = std::fs::OpenOptions::new().read(true).open(&path)?;
                file.read_to_string(commit_message)?;

                return Ok(());
            }

            return Ok(());
        }
        Err(e) => return Err(e),
    }
}
