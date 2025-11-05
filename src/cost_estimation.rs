use anyhow::{Context, Ok, Result};
use log::warn;
use tiktoken_rs::cl100k_base;

// Tuple for the cost estimate
pub type CostEstimate = (usize, f64);

/// Estimates the cost of an API request based on the input token count
/// Note: This uses INPUT token pricing only. Output tokens typically cost 2-4x more.
pub fn estimate_cost(model: &str, prompt: &str) -> Result<CostEstimate> {
    // Count tokens using the appropriate tokenizer
    let tokenizer = cl100k_base().context("Failed to load tokenizer")?;
    let token_count = tokenizer.encode_with_special_tokens(prompt).len();

    // Calculate cost based on model (INPUT token pricing per 1M tokens)
    // Prices sourced from OpenAI and Anthropic pricing as of January 2025
    let price_per_million = match model {
        // ===== OpenAI Models =====

        // GPT-5 family (latest generation, released August 2025)
        "gpt-5" | "gpt-5-chat-latest" => 1.25,
        "gpt-5-mini" => 0.25,
        "gpt-5-nano" => 0.05,

        // GPT-4o family (production models)
        "gpt-4o" | "chatgpt-4o-latest" => 2.50,
        "gpt-4o-2024-11-20" => 2.50,
        "gpt-4o-2024-08-06" => 2.50,
        "gpt-4o-2024-05-13" => 5.00, // Older version, higher price
        "gpt-4o-mini" | "gpt-4o-mini-2024-07-18" => 0.15,

        // GPT-4 Turbo family
        "gpt-4-turbo" | "gpt-4-turbo-2024-04-09" => 10.00,
        "gpt-4-turbo-preview" | "gpt-4-0125-preview" | "gpt-4-1106-preview" => 10.00,

        // GPT-4 base models (legacy, expensive)
        "gpt-4" | "gpt-4-0613" => 30.00,      // 8K context
        "gpt-4-32k" | "gpt-4-32k-0613" => 60.00, // 32K context

        // GPT-3.5 Turbo family (legacy, cost-effective)
        "gpt-3.5-turbo" | "gpt-3.5-turbo-0125" | "gpt-3.5-turbo-1106" => 0.50,
        "gpt-3.5-turbo-instruct" => 1.50,

        // Reasoning models (o-series) - expensive but powerful
        "o1" => 15.00,
        "o1-preview" | "o1-preview-2024-09-12" => 15.00,
        "o1-mini" | "o1-mini-2024-09-12" => 3.00,
        "o3-mini" => 1.10,
        "o3" => 10.00,
        "o3-pro" => 30.00,
        "o4-mini" => 1.10,

        // ===== Anthropic Claude Models =====

        // Claude Opus family (most powerful)
        "claude-opus-4.1" | "claude-opus-4-20250514" => 15.00,
        "claude-opus-4" | "claude-opus-4-20250104" => 15.00,
        "claude-opus-3" | "claude-3-opus-20240229" | "claude-3-opus-latest" => 15.00,

        // Claude Sonnet family (balanced performance)
        "claude-sonnet-4.5" | "claude-sonnet-4-5-20250929" => 3.00, // Standard context (≤200K)
        "claude-sonnet-4" | "claude-sonnet-4-20250104" => 3.00,
        "claude-sonnet-3.7" | "claude-3-7-sonnet-20250219" => 3.00,
        "claude-3-5-sonnet-20241022" | "claude-3-5-sonnet-latest" => 3.00,
        "claude-3-5-sonnet-20240620" => 3.00,

        // Claude Haiku family (fast and efficient)
        "claude-haiku-4.5" | "claude-haiku-4-5-20250416" => 1.00,
        "claude-haiku-3.5" | "claude-3-5-haiku-20241022" | "claude-3-5-haiku-latest" => 0.80,
        "claude-haiku-3" | "claude-3-haiku-20240307" => 0.25,

        // Default fallback for unknown models
        _ => {
            warn!("Unknown model '{}', using default pricing of $2.50 per 1M input tokens.", model);
            2.50 // Default to gpt-4o pricing
        }
    };

    let estimated_cost = (token_count as f64) * (price_per_million / 1_000_000.0);

    Ok((token_count, estimated_cost))
}

pub fn format_cost_estimate(cost_estimate: &CostEstimate) -> String {
    let (token_count, estimated_cost) = cost_estimate;

    format!(
        "Estimated cost: ${:.3} for processing {} tokens.",
        estimated_cost, token_count
    )
}
