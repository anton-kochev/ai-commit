// Updated prompt to request string format
pub fn get_prompt(diff: String) -> String {
    format!(
        r#"
        You are an expert commit message generator. Given a Git diff, produce a high-quality commit message as a single string formatted like this:

        {{Summary}}
        OR
        {{Summary}}\n\n{{Description}}

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
    )
}
