# Contributing to EpilogLite

Thank you for your interest in contributing to EpilogLite! This document provides guidelines and information for contributors.

## Code of Conduct

Be respectful, professional, and constructive in all interactions.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/epiloglite.git`
3. Create a new branch: `git checkout -b feature/your-feature-name`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m "Description of changes"`
7. Push to your fork: `git push origin feature/your-feature-name`
8. Open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building

```shell
cargo build
```

### Running Tests

```shell
cargo test
```

### Running Examples

```shell
cargo run --example basic_usage
```

## Code Style

EpilogLite follows Rust standard coding conventions with these specific requirements:

- **Indentation**: Use tabs (not spaces)
- **Naming**: Follow Rust naming conventions (snake_case for functions/variables, PascalCase for types)
- **Safety**: No `unsafe` code allowed (`unsafe_code = "forbid"` in Cargo.toml)
- **Documentation**: Public APIs must have doc comments
- **Tests**: New features must include tests

### Formatting

Run cargo fmt before committing:

```shell
cargo fmt
```

### Linting

Run clippy to check for common mistakes:

```shell
cargo clippy
```

## Testing Requirements

- All new code must include unit tests
- Aim for 90%+ test coverage
- Tests should be in the same file as the code they test (Rust convention)
- Integration tests go in the `tests/` directory

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test code here
        assert_eq!(result, expected);
    }
}
```

## Project Structure

```
epiloglite/
├── src/
│   ├── eplite/
│   │   ├── command/         # SQL command processing
│   │   ├── database.rs      # Database connection
│   │   ├── error.rs         # Error types
│   │   ├── os/             # OS abstraction layer
│   │   ├── persistence/    # Storage engine
│   │   ├── types/          # Data types
│   │   └── utility.rs      # Utility functions
│   ├── eplite.rs           # Main module
│   └── lib.rs              # Library entry point
├── examples/               # Example programs
├── design/                 # Design documentation
└── tests/                  # Integration tests
```

## Architecture Overview

EpilogLite is organized into several key modules:

1. **Command**: SQL parsing and bytecode generation
2. **Persistence**: B-tree storage and page management
3. **OS**: Cross-platform file I/O and system calls
4. **Database**: High-level database API

See [design/ARCHITECTURE.md](design/ARCHITECTURE.md) for details.

## Pull Request Guidelines

### Before Submitting

- [ ] Code compiles without warnings
- [ ] All tests pass
- [ ] New features include tests
- [ ] Documentation is updated
- [ ] Code is formatted with `cargo fmt`
- [ ] No clippy warnings

### PR Description

Include:
- Description of changes
- Motivation and context
- Related issue numbers (if any)
- Testing performed
- Any breaking changes

### Review Process

1. PRs require at least one approval
2. All CI checks must pass
3. Maintainers may request changes
4. Once approved, PRs will be merged by maintainers

## Areas for Contribution

### High Priority

- SQL parser implementation
- B-tree storage engine
- Virtual machine bytecode execution
- Transaction support

### Medium Priority

- Query optimizer
- Index management
- Performance improvements
- Additional tests

### Documentation

- API documentation
- Tutorials and guides
- Design documentation updates
- Example programs

## Reporting Issues

### Bug Reports

Include:
- Description of the bug
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information (OS, Rust version)
- Relevant code snippets

### Feature Requests

Include:
- Description of the feature
- Use cases
- Potential implementation approach
- Examples of similar features elsewhere

## Questions and Discussions

Use [GitHub Discussions](https://github.com/jeleniel/epiloglite/discussions) for:
- Questions about using EpilogLite
- Design discussions
- General feedback
- Ideas and suggestions

## Security Issues

Report security vulnerabilities privately using [GitHub's security advisory feature](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability).

Do NOT open public issues for security problems.

## License

By contributing to EpilogLite, you agree that your contributions will be licensed under the GNU Lesser General Public License 3.0.

## Recognition

Contributors will be acknowledged in the project documentation and release notes.

## Getting Help

- Check the [documentation](design/ARCHITECTURE.md)
- Search [existing issues](https://github.com/jeleniel/epiloglite/issues)
- Ask in [discussions](https://github.com/jeleniel/epiloglite/discussions)
- Read the [design documents](design/)

Thank you for contributing to EpilogLite!
