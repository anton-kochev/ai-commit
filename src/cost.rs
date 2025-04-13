use log::info;
use std::env;
use tiktoken_rs::cl100k_base;

/// Estimates the cost of an API request based on the input token count
pub fn estimate_cost(prompt: &str, model: &str) -> (usize, f64) {
    // Count tokens using the appropriate tokenizer
    let tokenizer = cl100k_base().unwrap();
    let token_count = tokenizer.encode_with_special_tokens(prompt).len();

    // Calculate cost based on model
    let price_per_1000 = match model {
        "gpt-4" => 0.03,
        "gpt-3.5-turbo" => 0.0015,
        _ => 0.03, // default fallback
    };

    let estimated_cost = (token_count as f64) * (price_per_1000 / 1000.0);

    (token_count, estimated_cost)
}

/// Prompts the user for confirmation based on the token count and estimated cost
pub fn prompt_for_confirmation(token_count: usize, estimated_cost: f64) -> bool {
    info!("Input size: {} tokens (approx. ${:.3}). Proceed with this request? [Y/n]",
          token_count, estimated_cost);

    let skip_confirmation = env::var("AI_COMMIT_SKIP_COST_CONFIRM").is_ok();
    if skip_confirmation {
        info!("Skipping cost confirmation due to AI_COMMIT_SKIP_COST_CONFIRM environment variable");
        return true;
    }

    // Use dialoguer to prompt the user
    let proceed = dialoguer::Confirm::new()
        .with_prompt("")
        .default(true)
        .interact()
        .unwrap_or(false);

    if !proceed {
        info!("Request canceled by the user.");
    }

    proceed
}
