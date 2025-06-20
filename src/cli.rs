/// The options presented to the user.
pub enum UserChoice {
    Edit,
    Commit,
    Cancel,
}

/// Prompt the user with the suggested commit message and give options for action.
pub fn prompt_user_for_action() -> UserChoice {
    let options = &["Commit", "Edit and Commit", "Cancel"];

    // Use dialoguer to create a selection prompt
    let selection = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(options)
        .default(2)
        .interact()
        .unwrap_or(1);

    match selection {
        0 => UserChoice::Commit,
        1 => UserChoice::Edit,
        _ => UserChoice::Cancel,
    }
}

/// Prompt the user for confirmation before proceeding with the operation.
pub fn prompt_for_confirmation(confirmation: &str) -> bool {
    dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(confirmation)
        .default(false)
        .interact()
        .unwrap_or(false)
}
