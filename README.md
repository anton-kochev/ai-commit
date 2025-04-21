# AI Commit

A Rust-based tool that uses OpenAI's API to generate high-quality Git commit messages based on your staged changes.

## Features

- Automatically generates commit messages based on your staged changes
- Interactive CLI to accept, regenerate, or cancel commit messages
- Cost estimation and confirmation before making API calls
- Support for ignoring files via `.ai-commit-ignore`
- Yes/No selection dialog for user interactions

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

AI Commit primarily uses command-line arguments for configuration. However, it also caches certain settings like the model choice and API keys in a configuration file for future use.

- **Model**: Specify the model using the `-m` or `--model` flag. This choice is cached.
- **API Key**: Provide your API key using the `-k` or `--api-key` flag in the format `<provider>=<key>`. This is also cached.

Supported Providers:

- `openai`
- `anthropic`

Supported Models:

- `gpt-3.5-turbo`
- `gpt-4`
- `gpt-4.1`
- `gpt-4.1-mini`
- `gpt-4.1-nano`
- `gpt-4.5-preview`
- `gpt-4o`
- `gpt-4o-mini`
- `o1`
- `o1-mini`
- `o1-pro`
- `o3`
- `o3-mini`
- `o4-mini`

## Command-line Options

AI Commit supports the following command-line options:

- `-m <model>`, `--model <model>`: Specify the model to use (e.g., `gpt-4o`). This value is cached.
- `-k <provider>=<key>`, `--api-key <provider>=<key>`: Specify the API key provider and key (e.g., `openai=sk-yourkey`). This value is cached.
- `--estimate-only`: Run to approximately estimate the token count and cost only
- `--help`: Show help information

Example:

```bash
ai-commit -m gpt-4o -k openai=sk-yourkey
```

### Estimate-only Mode

When running in estimate-only mode, AI Commit will:

1. Calculate the estimated token count and cost
2. Display this information to the user

This allows you to test the cost estimation without making actual API calls or committing changes.

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
