# Contributing to Mutant-Kraken

Thank you for your interest in contributing to Mutant-Kraken! Your help is highly appreciated. Please follow these guidelines to contribute to the project.

## Table of Contents

1. [Code of Conduct](#code-of-conduct)
2. [How to Contribute](#how-to-contribute)
   - [Reporting Bugs](#reporting-bugs)
   - [Suggesting Enhancements](#suggesting-enhancements)
   - [Submitting Pull Requests](#submitting-pull-requests)
3. [Development Guidelines](#development-guidelines)
   - [Setting Up the Development Environment](#setting-up-the-development-environment)
   - [Running Tests](#running-tests)
   - [Commit Message Guidelines](#commit-message-guidelines)

## Code of Conduct

Please read our [Code of Conduct](CODE_OF_CONDUCT.md) to understand the standards we expect from all contributors.

## How to Contribute

### Reporting Bugs

If you find a bug, please report it by opening an issue on our [GitHub Issues](https://github.com/JosueMolinaMorales/mutant-kraken/issues) page. Provide as much detail as possible, including:

- Steps to reproduce the bug.
- Expected and actual behavior.
- Screenshots or logs if applicable.
- Environment details (OS, Kotlin version, Rust version, etc.).

### Suggesting Enhancements

If you have an idea for an improvement or a new feature, please submit it as an issue. Clearly explain:

- The problem your suggestion solves.
- A detailed description of the enhancement or feature.
- Any relevant examples, code snippets, or references.

### Submitting Pull Requests

Before submitting a pull request, please ensure you:

1. Fork the repository and create your branch from `main`.
2. Follow the [Development Guidelines](#development-guidelines).
3. Test your changes thoroughly.
4. Write clear, concise commit messages (see [Commit Message Guidelines](#commit-message-guidelines)).
5. Update documentation and add tests as necessary.

When ready, submit your pull request. Describe your changes in detail, and link any related issues.

## Development Guidelines

### Setting Up the Development Environment

1. Clone the repository:
   ```sh
   git clone https://github.com/JosueMolinaMorales/mutant-kraken.git
   cd mutant-kraken
   ```

2. Install Rust and Kotlin if not already installed:
   - [Rust Installation Guide](https://www.rust-lang.org/learn/get-started)
   - [Kotlin Installation Guide](https://kotlinlang.org/docs/command-line.html)

3. Build the project:
   ```sh
   cargo build
   ```

### Running Tests

To run the tests, use:
```sh
cargo test
```

Ensure all tests pass before submitting your pull request.

### Commit Message Guidelines

- Use the present tense ("Add feature" not "Added feature").
- Capitalize the first letter of the commit message.
- Use the imperative mood ("Move button left..." not "Moves button left...").
- Limit the subject line to 50 characters or less.
- Include references to issues or pull requests when applicable (e.g., `Fixes #123`).

### Changelog
Please ensure that any changes you make are listed out in the `CHANGELOG.md` file.

## Thank You!

Your contributions make Mutant-Kraken better for everyone. Thank you for taking the time to contribute!
