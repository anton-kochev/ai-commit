use log::{info, trace};
use std::env;
use tiktoken_rs::cl100k_base;

// Tuple for the cost estimate
pub type CostEstimate = (usize, f64);

/// Estimates the cost of an API request based on the input token count
pub fn estimate_cost(model: &str, prompt: &str) -> CostEstimate {
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
            trace!("Unknown model '{}', using default pricing", model);
            0.001 // Default to $1.00 per 1M tokens = $0.001 per 1K tokens
        }
    };

    let estimated_cost = (token_count as f64) * (price_per_1000 / 1000.0);

    (token_count, estimated_cost)
}

pub fn print_cost(cost_estimate: &CostEstimate) {
    let (token_count, estimated_cost) = cost_estimate;
    info!(
        "Estimated cost: ${:.3} for processing {} tokens.",
        estimated_cost, token_count
    );
}
