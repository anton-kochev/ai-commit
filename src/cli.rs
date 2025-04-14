use dialoguer::Select;

/// The options presented to the user.
pub enum UserChoice {
    Commit,
    Regenerate,
    Cancel,
}

/// Prompt the user with the suggested commit message and give options for action.
pub fn prompt_user_for_action(commit_msg: &str) -> UserChoice {
    // Display the suggested commit message as a single string
    println!("{}", commit_msg.trim());
    println!();

    let options = &["✅ Use this message", "🔄 Regenerate message", "❌ Cancel"];

    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(options)
        .default(2)
        .interact()
        .unwrap_or(2);

    match selection {
        0 => UserChoice::Commit,
        1 => UserChoice::Regenerate,
        2 | _ => UserChoice::Cancel,
    }
}
