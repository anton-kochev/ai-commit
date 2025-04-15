use dialoguer::Select;

/// The options presented to the user.
pub enum UserChoice {
    Commit,
    Cancel,
}

/// Prompt the user with the suggested commit message and give options for action.
pub fn prompt_user_for_action(commit_msg: &str) -> UserChoice {
    // Display the suggested commit message as a single string
    println!("{}", commit_msg.trim());
    println!();

    let options = &["✅ Commit", "❌ Cancel"];

    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(options)
        .default(2)
        .interact()
        .unwrap_or(2);

    match selection {
        0 => UserChoice::Commit,
        _ => UserChoice::Cancel,
    }
}
