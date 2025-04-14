use log::trace;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;
use serde_json::json;

use crate::cost;

/// Structs for deserializing the OpenAI Chat Completions response.
#[derive(Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessageResponse,
}

#[derive(Deserialize)]
pub struct ChatMessageResponse {
    pub content: String,
}

/// Generates a commit message by sending the provided diff to the OpenAI ChatGPT API.
/// If dry_run is true, it will trace the request instead of making an actual API call.
/// Returns the commit message as a string.
pub fn generate_commit_message(
    diff: &str,
    dry_run: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    if dry_run {
        trace!(
            "[DRY RUN] Would send request to OpenAI API with diff: {}",
            diff
        );
        // Return a mock commit message in dry-run mode
        return Ok(
            "Dry run commit message\n\nThis is a mock commit message generated in dry run mode."
                .to_string(),
        );
    }

    // Get API key from environment
    let api_key = std::env::var("OPENAI_API_KEY")?;

    // Create a client with increased timeout
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(60)) // Increase timeout to 60 seconds
        .build()?;

    // Define the model directly in the code
    let model = "o1"; //"gpt-4.1-mini";
    trace!("Using OpenAI model: {}", model);

    // Updated prompt to request string format
    let prompt = format!(
        r#"
        You are an expert commit message generator. Given a Git diff, produce a high-quality commit message as a single string formatted like this:

        "{{Summary}}"
        OR
        "{{Summary}}\n\n{{Description}}"

        Guidelines:
        - "Summary" should be a one-line description of the key change, starting with a capital letter.
        - If the change needs more detail, add "Description" on a new paragraph (after a double newline). It should also start with a capital letter and provide extra insight without duplicating the summary.
        - Skip trivial changes (e.g., formatting, comments) and keep the output focused.
        - Return only the resulting string without any extra text.
        - Use dash points.

        Git diff:
        {}
        "#,
        diff
    );

    // Estimate cost before proceeding
    let (token_count, estimated_cost) = cost::estimate_cost(&prompt, &model);

    // Prompt user for confirmation
    if !cost::prompt_for_confirmation(token_count, estimated_cost) {
        return Err("User canceled the operation".into());
    }

    // Build the JSON request body.
    let request_body = json!({
       "model": model,
       "messages": [
            { "role": "user", "content": prompt }
       ]
    });

    // Send the POST request to the OpenAI Chat Completions API.
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .header(CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()?;

    // Check if the API call was successful.
    if !response.status().is_success() {
        return Err(format!("API request failed with status: {}", response.status()).into());
    }

    // Deserialize the JSON response.
    let json_response: ChatResponse = response.json()?;
    let content = json_response
        .choices
        .get(0)
        .map(|choice| choice.message.content.clone())
        .ok_or("No response from API")?;

    // Return the trimmed content
    Ok(content.trim().to_string())
}
