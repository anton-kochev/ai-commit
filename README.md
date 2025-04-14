# AI Commit

A Rust-based tool that uses OpenAI's API to generate high-quality Git commit messages based on your staged changes.

## Features

- Automatically generates commit messages based on your staged changes
- Interactive CLI to accept, regenerate, or cancel commit messages
- Cost estimation and confirmation before making API calls
- Support for ignoring files via `.ai-commit-ignore`
- Dry-run mode for testing without making API calls

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))
- Git
- OpenAI API key

### Building from source

```bash
git clone https://github.com/yourusername/ai-commit.git
cd ai-commit
cargo build --release
```

The binary will be available at `target/release/ai-commit`.

## Usage

1. Stage your changes with Git:

   ```bash
   git add <files>
   ```

2. Run AI Commit:

   ```bash
   ai-commit
   ```

3. Review the suggested commit message and choose to:
   - Accept the message and commit
   - Regenerate a new message
   - Cancel the operation

## Configuration

AI Commit can be configured using environment variables:

- `OPENAI_API_KEY`: Your OpenAI API key (required)
- `AI_COMMIT_MODEL`: The OpenAI model to use (default: "gpt-3.5-turbo-16k")
- `DRY_RUN`: Set to any value to enable dry-run mode (no API calls)
- `AI_COMMIT_SKIP_COST_CONFIRM`: Set to any value to skip cost confirmation
- `RUST_LOG`: Set the log level (default: "info")

You can set these in a `.env` file in your project root:

``` plaintext
OPENAI_API_KEY=your-api-key-here
AI_COMMIT_MODEL=gpt-4-turbo
```

## Ignoring Files

Create a `.ai-commit-ignore` file in your repository root to specify files that should be ignored when generating commit messages. The format is similar to `.gitignore`:

``` plaintext
# Ignore specific files
.env
*.lock

# Ignore specific directories
node_modules/
target/
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
