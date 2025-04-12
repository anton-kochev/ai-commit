use dialoguer::Select;
use crate::api;

/// The options presented to the user.
pub enum UserChoice {
    Commit,
    Regenerate,
    Cancel,
}

/// Prompt the user with the suggested commit message and give options for action.
pub fn prompt_user_for_action(commit_msg: &api::CommitMessage) -> UserChoice {
    // Display the suggested commit message in a formatted way
    println!("\nSuggested commit message:");
    println!("Summary: {}", commit_msg.summary);
    if !commit_msg.description.is_empty() {
        println!("\nDescription:\n{}", commit_msg.description);
    }
    println!();

    let options = &[
        "✅ Use this message",
        "🔄 Regenerate message",
        "❌ Cancel",
    ];

    let selection = Select::new()
        .with_prompt("What would you like to do?")
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(2);

    match selection {
        0 => UserChoice::Commit,
        1 => UserChoice::Regenerate,
        2 | _ => UserChoice::Cancel,
    }
}
