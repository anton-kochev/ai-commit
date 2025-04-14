use log::info;
use std::env;
use tiktoken_rs::cl100k_base;

/// Estimates the cost of an API request based on the input token count
pub fn estimate_cost(prompt: &str, model: &str) -> (usize, f64) {
    // Count tokens using the appropriate tokenizer
    let tokenizer = cl100k_base().unwrap();
    let token_count = tokenizer.encode_with_special_tokens(prompt).len();

    // Calculate cost based on model
    // Prices are in dollars per 1K tokens
    let price_per_1000 = match model {
        // GPT-4.1 models
        "gpt-4.1" => 0.001, // $1.00 per 1M tokens
        // GPT-4.1 Mini models
        "gpt-4.1-mini" => 0.0002, // $0.20 per 1M tokens
        // GPT-4.1 Nano models
        "gpt-4.1-nano" => 0.00005, // $0.05 per 1M tokens
        // GPT-4.5 Preview models
        "gpt-4.5-preview" => 0.0375, // $37.50 per 1M tokens
        // GPT-4o models
        "gpt-4o" => 0.00125, // $1.25 per 1M tokens
        // GPT-4o Mini models
        "gpt-4o-mini" => 0.000075, // $0.075 per 1M tokens
        // O1 models
        "o1" => 0.0075, // $7.50 per 1M tokens
        // O1 Pro models
        "o1-pro" => 0.075, // $75.00 per 1M tokens
        // O3 Mini models
        "o3-mini" => 0.00055, // $0.55 per 1M tokens
        // O1 Mini models
        "o1-mini" => 0.00055, // $0.55 per 1M tokens
        // Computer Use Preview models
        "computer-use-preview" => 0.0015, // $1.50 per 1M tokens
        // Legacy models (keeping for backward compatibility)
        "gpt-4" => 0.03,
        "gpt-3.5-turbo" => 0.0015,

        // Default fallback for unknown models
        _ => {
            info!("Unknown model '{}', using default pricing", model);
            0.001 // Default to $1.00 per 1M tokens = $0.001 per 1K tokens
        }
    };

    let estimated_cost = (token_count as f64) * (price_per_1000 / 1000.0);

    (token_count, estimated_cost)
}

/// Prompts the user for confirmation based on the token count and estimated cost
pub fn prompt_for_confirmation(token_count: usize, estimated_cost: f64) -> bool {
    info!(
        "Input size: {} tokens (approx. ${:.3}). Proceed with this request? [Y/N]",
        token_count, estimated_cost
    );

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
