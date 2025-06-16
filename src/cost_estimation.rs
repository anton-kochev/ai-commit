use anyhow::{Context, Ok, Result};
use log::warn;
use tiktoken_rs::cl100k_base;

// Tuple for the cost estimate
pub type CostEstimate = (usize, f64);

/// Estimates the cost of an API request based on the input token count
pub fn estimate_cost(model: &str, prompt: &str) -> Result<CostEstimate> {
    // Count tokens using the appropriate tokenizer
    let tokenizer = cl100k_base().context("Failed to load tokenizer")?;
    let token_count = tokenizer.encode_with_special_tokens(prompt).len();

    // Calculate cost based on model
    // Prices are in dollars per 1K tokens
    let price_per_1000 = match model {
        "gpt-4.1" => 0.001,          // $1.00 per 1M tokens
        "gpt-4.1-mini" => 0.0002,    // $0.20 per 1M tokens
        "gpt-4.1-nano" => 0.00005,   // $0.05 per 1M tokens
        "gpt-4.5-preview" => 0.0375, // $37.50 per 1M tokens
        "gpt-4o" => 0.00125,         // $1.25 per 1M tokens
        "gpt-4o-mini" => 0.000075,   // $0.075 per 1M tokens
        "o1" => 0.0075,              // $7.50 per 1M tokens
        "o1-mini" => 0.00055,        // $0.55 per 1M tokens
        "o1-pro" => 0.075,           // $75.00 per 1M tokens
        "o3" => 0.005,               // $5 per 1M tokens
        "o3-mini" => 0.00055,        // $0.55 per 1M tokens
        "o4-mini" => 0.0011,         // $1.10 per 1M tokens
        // Computer Use Preview models
        "computer-use-preview" => 0.0015, // $1.50 per 1M tokens
        // Legacy models (keeping for backward compatibility)
        "gpt-4" => 0.03,
        "gpt-3.5-turbo" => 0.0015,

        // Default fallback for unknown models
        _ => {
            warn!("Unknown model, using default pricing.");
            0.001 // Default to $1.00 per 1M tokens = $0.001 per 1K tokens
        }
    };

    let estimated_cost = (token_count as f64) * (price_per_1000 / 1000.0);

    Ok((token_count, estimated_cost))
}

pub fn format_cost_estimate(cost_estimate: &CostEstimate) -> String {
    let (token_count, estimated_cost) = cost_estimate;

    format!(
        "Estimated cost: ${:.3} for processing {} tokens.",
        estimated_cost, token_count
    )
}
