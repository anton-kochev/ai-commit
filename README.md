# AI Commit

A Rust-based tool that uses OpenAI's API to generate high-quality Git commit messages based on your staged changes.

## Features

- Automatically generates commit messages based on your staged changes
- Interactive CLI for reviewing and confirming commit messages
- Interactive commit message editing with your preferred editor
- Cost estimation and confirmation before making API calls
- Support for ignoring files via `.ai-commit-ignore`
- Configuration caching for models and API keys
- Support for multiple AI providers (currently OpenAI)
- Token-based cost calculation using tiktoken
- Sensitive information detection with warnings
- Structured commit message format with optional descriptions

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

### Installing from Binary

After building, you can install the binary to your system:

```bash
cargo install --path .
```

Or copy the binary to a directory in your PATH:

```bash
cp target/release/ai-commit /usr/local/bin/
```

### Installing via Homebrew (macOS)

If you're on macOS, you can install AI Commit using Homebrew:

```bash
brew tap anton-kochev/ai-commit
brew install ai-commit
```

The repository is available at [homebrew-ai-commit](https://github.com/anton-kochev/homebrew-ai-commit)

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
   - Edit the message with your preferred editor and commit
   - Cancel the operation

## Configuration

AI Commit primarily uses command-line arguments for configuration. However, it also caches certain settings like the model choice and API keys in a configuration file (`~/.config/ai-commit/config.json`) for future use.

- **Model**: Specify the model using the `-m` or `--model` flag. This choice is cached.
- **API Key**: Provide your API key using the `-k` or `--api-key` flag in the format `<provider>=<key>`. This is also cached.
- **Context**: Provide additional context using the `-c` or `--context` flag (e.g., issue numbers, descriptions).

Supported Providers:

- `openai`

Note: While the configuration supports provider specification for future extensibility, currently only OpenAI is implemented.

## Command-line Options

AI Commit supports the following command-line options:

- `-m <model>`, `--model <model>`: Specify the model to use (e.g., `gpt-4o`). This value is cached.
- `-k <provider>=<key>`, `--api-key <provider>=<key>`: Specify the API key provider and key (e.g., `openai=sk-yourkey`). This value is cached.
- `-c <context>`, `--context <context>`: Provide additional context for the commit message (e.g., issue numbers, descriptions).
- `--help`: Show help information

Example:

```bash
ai-commit -m gpt-4o -k openai=sk-yourkey
```

## Environment Variables

AI Commit supports the following environment variables:

- `RUST_LOG`: Controls the logging level (e.g., `trace`, `debug`, `info`, `warn`, `error`). Defaults to `info`.
- `EDITOR`: Specifies the text editor to use when editing commit messages. Defaults to `nano` if not set.

Example:

```bash
RUST_LOG=debug ai-commit
```

```bash
EDITOR=vim ai-commit
```

## Ignoring Files

Create a `.ai-commit-ignore` file in your repository root to specify files that should be ignored when generating commit messages. The format is similar to `.gitignore`:

```plaintext
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
