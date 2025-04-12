# ai-commit TODO Checklist

This checklist outlines each task needed to implement the `ai-commit` tool. Each section corresponds to a major module or integration step. Check items off as you complete them.

---

## 1. Project Setup & Logging

- [ ] **Scaffold Project:**
  - [ ] Run `cargo init ai-commit` to create a new Rust project.
  - [ ] Set up a basic project structure.

- [ ] **Configure Dependencies (Initial):**
  - [ ] Update `Cargo.toml` to include:
    - `log = "0.4"`
    - `env_logger = "0.9"`

- [ ] **Initialize Logging:**
  - [ ] In `src/main.rs`, import `env_logger` and `log` macros.
  - [ ] Call `env_logger::init()` at the start of `main()`.
  - [ ] Log a startup message (e.g., `info!("Starting ai-commit");`).

---

## 2. Git Module – Retrieve Staged Diff

- [ ] **Add Dependency:**
  - [ ] Add `git2 = "0.16"` to `Cargo.toml`.

- [ ] **Create `src/git.rs`:**
  - [ ] Create a module file `src/git.rs` and include it in `main.rs` (using `mod git;`).

- [ ] **Implement Diff Retrieval:**
  - [ ] Write a function `pub fn get_staged_diff() -> Result<String, Box<dyn std::error::Error>>` that:
    - [ ] Opens the current Git repository (`Repository::open(".")`).
    - [ ] Validates that the repository is available (error if not).
    - [ ] Retrieves HEAD tree (or handles initial commit scenario).
    - [ ] Obtains the index and computes the diff using `repo.diff_tree_to_index`.
    - [ ] Iterates over the diff (using a callback with `diff.print`) to build a unified diff string.
    - [ ] Returns the diff as a `String`.

- [ ] **Testing:**
  - [ ] Stage some changes and run the tool.
  - [ ] Confirm that the diff is correctly printed or logged.

---

## 3. Ignore Module – Load Ignore Patterns

- [ ] **Add Dependency:**
  - [ ] Add `globset = "0.4"` to `Cargo.toml`.

- [ ] **Create `src/ignore.rs`:**
  - [ ] Create a new module file `src/ignore.rs` and include it in `main.rs` (using `mod ignore;`).

- [ ] **Implement Pattern Loading:**
  - [ ] Write a function `pub fn load_ignore_patterns(repo_path: &std::path::Path) -> Result<globset::GlobSet, Box<dyn std::error::Error>>` that:
    - [ ] Checks for a `.gitignore` (or `.gpt-commit-ignore`) file in the repository root.
    - [ ] Reads the file line by line.
    - [ ] Ignores blank lines and comments.
    - [ ] Compiles valid patterns into a `GlobSet` using `GlobSetBuilder`.
    - [ ] Returns a built `GlobSet` (if file not found, return an empty set without error).

- [ ] **Testing:**
  - [ ] Verify that known ignore patterns match intended paths.
  - [ ] Log the count or sample of loaded patterns for confirmation.

---

## 4. Integrate Ignore Filtering into Diff Generation

- [ ] **Modify Diff Function:**
  - [ ] In `git::get_staged_diff`, load ignore patterns by calling `ignore::load_ignore_patterns`.
  - [ ] For each file in the diff callback:
    - [ ] Convert the file path to a consistent string format.
    - [ ] Check if it matches any ignore patterns from the `GlobSet`.
    - [ ] Skip appending diff content for ignored files.

- [ ] **Testing:**
  - [ ] Stage changes in both ignored and non-ignored files.
  - [ ] Verify that only non-ignored file changes appear in the diff string.

---

## 5. OpenAI API Module – Generate Commit Message

- [ ] **Add Dependencies:**
  - [ ] Update `Cargo.toml` with:
    - `reqwest = { version = "0.11", features = ["blocking", "json"] }`
    - `serde = { version = "1.0", features = ["derive"] }`
    - `serde_json = "1.0"`
    - `dotenv = "0.15"`

- [ ] **Create `src/api.rs`:**
  - [ ] Create a new module file `src/api.rs` and include it in `main.rs` (using `mod api;`).

- [ ] **Implement API Call:**
  - [ ] Write a function `pub fn generate_commit_message(diff: &str) -> Result<String, Box<dyn std::error::Error>>` that:
    - [ ] Loads the OpenAI API key from the environment (using `dotenv::dotenv()` and `std::env::var("OPENAI_API_KEY")`).
    - [ ] Constructs a JSON payload:
      - Model: `"gpt-4"`.
      - Messages: one user message incorporating the diff.
    - [ ] Sends a POST request to `https://api.openai.com/v1/chat/completions` using the blocking client.
    - [ ] Parses the JSON response into defined structs (e.g., `ChatResponse`, `ChatChoice`, `ChatMessage`).
    - [ ] Extracts the commit message content from the response.
    - [ ] Returns the commit message.

- [ ] **Testing:**
  - [ ] Run the tool with a valid API key and a sample diff.
  - [ ] Verify that a commit message is received and displayed.

---

## 6. CLI Module – Interactive Confirmation / Regeneration

- [ ] **Add Dependency:**
  - [ ] Add `dialoguer = "0.10"` to `Cargo.toml`.

- [ ] **Create `src/cli.rs`:**
  - [ ] Create a new module file `src/cli.rs` and include it in `main.rs` (using `mod cli;`).

- [ ] **Implement User Prompt:**
  - [ ] Define an enum `UserChoice` with variants: `Commit`, `Regenerate`, and `Cancel`.
  - [ ] Implement a function `pub fn prompt_user_for_action(suggested_message: &str) -> UserChoice` that:
    - [ ] Displays the suggested commit message.
    - [ ] Uses `dialoguer::Select` to present options:
      - "✅ Use this message"
      - "🔄 Regenerate message"
      - "❌ Cancel"
    - [ ] Returns the selected `UserChoice`.

- [ ] **Integrate Interactive Loop in Main:**
  - [ ] In `main.rs`, after generating the commit message:
    - [ ] Enter a loop where:
      - If the user selects **Regenerate**, call the OpenAI API again.
      - If **Commit** is selected, break and continue.
      - If **Cancel** is selected, exit the application gracefully.

- [ ] **Testing:**
  - [ ] Verify that the interactive prompt correctly displays the message and options.
  - [ ] Test each branch (commit, regenerate, cancel) and ensure proper behavior.

---

## 7. Git Module – Finalize Commit Creation

- [ ] **Extend `src/git.rs`:**
  - [ ] Implement a function `pub fn commit_with_message(message: &str) -> Result<(), git2::Error>` that:
    - [ ] Checks that there are staged changes (validate non-empty index).
    - [ ] Writes the index to a tree.
    - [ ] Retrieves the current HEAD commit (if it exists) for parent information.
    - [ ] Creates a commit signature using `repo.signature()`.
    - [ ] Uses `repo.commit(...)` to create the commit with the provided message.
    - [ ] Returns `Ok(())` upon success or an error otherwise.

- [ ] **Integrate Commit Function in Main Flow:**
  - [ ] After the user confirms the commit message, call `git::commit_with_message`.
  - [ ] Log success or errors accordingly.

- [ ] **Testing:**
  - [ ] Perform a full flow commit and confirm that the commit appears in the Git history (using `git log`).
  - [ ] Test error cases such as missing user configuration or an empty index.

---

## 8. Final Integration & End-to-End Testing

- [ ] **In `main.rs`, integrate all modules in the proper sequence:**
  - [ ] Initialize logging.
  - [ ] Check for a valid Git repository.
  - [ ] Get the staged diff (with ignore filtering).
  - [ ] If diff is empty, log an error and exit.
  - [ ] Generate the commit message using the API module.
  - [ ] Enter the interactive confirmation loop (via the CLI module).
  - [ ] On confirmation, create the commit using the Git module.

- [ ] **Test Scenarios:**
  - [ ] No staged changes should result in an error message.
  - [ ] Successful generation and commit when staged changes exist.
  - [ ] Regeneration option should fetch a new commit message.
  - [ ] Cancellation should exit the application gracefully.
  - [ ] Simulate API errors and verify error handling.

---

## 9. Documentation & Future Enhancements

- [ ] **Update README:**
  - [ ] Document the tool’s purpose and usage.
  - [ ] Provide instructions on setting the `OPENAI_API_KEY`.
  - [ ] Explain the expected structure of the ignore file.

- [ ] **Clean Up:**
  - [ ] Remove any debug logs or temporary prints.
  - [ ] Ensure all error messages are user-friendly.

- [ ] **Future Enhancements:**
  - [ ] Make the AI model (GPT-3.5 / GPT-4) configurable via CLI flags.
  - [ ] Allow for manual editing of the generated commit message before commit.
  - [ ] Implement additional command-line options (e.g., `--dry-run`, `--no-confirm`).
  - [ ] Cache the last generated commit message.
  - [ ] Extend ignore pattern support (e.g., auto-generation of `.gpt-commit-ignore`).

---

## 10. Final Review

- [ ] **Run through the entire workflow multiple times** to ensure:
  - Every module works individually.
  - The integration has no orphaned or hanging code.
  - Logging provides sufficient insight into the tool’s operations.
- [ ] **Check cross-platform compatibility** (especially if testing on Windows vs. Unix-based systems).
