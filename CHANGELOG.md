# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.1] - 2025-11-17

### Changed
- Refactored commit message generation system for improved clarity and maintainability
- Updated cost estimation pricing for Claude Haiku model (new version pricing)
- Enhanced Git diff formatting in API prompts with markdown code blocks for better readability
- Improved system prompt wording to enhance commit message generation accuracy

### Documentation
- Added build and release workflow badges to README
- Updated README to clarify Anthropic AI provider support

## [0.7.0] - 2025-11-14

### Added
- `--context-lines` CLI flag to control the number of context lines in git diffs (default: 10)
  - Allows users to balance between providing sufficient context and managing API token costs
  - Higher values provide more surrounding code context to the AI

### Changed
- Completely restructured system prompt for improved AI effectiveness and consistency
  - Reduced prompt size by 60% while maintaining all functionality
  - Implemented XML-tagged structure based on Anthropic's Claude 4 best practices
  - Enhanced guidelines to focus on purpose and impact over implementation details
  - Improved instruction clarity with positive framing and better examples
- Updated README.md to document the `--context-lines` option

[Unreleased]: https://github.com/anton-kochev/ai-commit/compare/v0.7.1...HEAD
[0.7.1]: https://github.com/anton-kochev/ai-commit/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/anton-kochev/ai-commit/compare/v0.6.2...v0.7.0
