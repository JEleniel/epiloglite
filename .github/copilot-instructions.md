# Copilot Instructions for EpilogLite

## Project Overview

EpilogLite is a Rust implementation of a database engine inspired by SQLite. This repository contains the complete source code for the EpilogLite database engine, including test scripts.

## Technology Stack

-   **Language**: Rust (Edition 2021)
-   **Build System**: Cargo
-   **License**: GNU Lesser General Public License 3.0 only (LGPL-3.0-only)
-   **Key Features**: Database engine, SQL support, relational database

## Build and Test Commands

### Building the Project

```bash
# Check the project for errors
cargo check

# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

### Linting

```bash
# Run clippy for linting
cargo clippy

# Format code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check
```

## Code Style Guidelines

### Rust Conventions

1. **Safety**: The project has `unsafe_code = "forbid"` in `Cargo.toml`. Never introduce unsafe code.
2. **Naming Conventions**:
    - Use UpperCamelCase for types, structs, enums, and variants
    - Use snake_case for functions, variables, and modules
    - Follow Rust standard naming conventions
3. **Code Organization**: The project is modular in design. Maintain this modularity.

### Documentation

1. Always add doc comments (`///`) for public APIs
2. Reference existing documentation in the `design/` folder when working on architecture-related changes
3. Keep documentation up-to-date with code changes

## Project Structure

```text
epiloglite/
├── src/              # Source code
│   ├── eplite/       # Core database engine implementation
│   └── lib.rs        # Library entry point
├── design/           # Architecture and design documentation
│   ├── ARCHITECTURE.md
│   ├── FILEFORMAT.md
│   ├── QUERYPLANNER.md
│   ├── TRANSACTIONS.md
│   ├── VIRTUALMACHINE.md
│   └── sql_syntax/   # SQL syntax documentation
├── artwork/          # Project artwork
└── Cargo.toml        # Rust package manifest
```

## Important Documentation

When working on specific components, reference these design documents:

-   **Architecture**: `design/ARCHITECTURE.md` - Overall system design
-   **File Format**: `design/FILEFORMAT.md` - Database file format specification
-   **Query Planner**: `design/QUERYPLANNER.md` - Query optimization and planning
-   **Virtual Machine**: `design/VIRTUALMACHINE.md` - Bytecode execution engine
-   **Transactions**: `design/TRANSACTIONS.md` - Transaction handling

## Best Practices

### Code Changes

1. **Minimal Changes**: Make the smallest possible changes to achieve the goal
2. **Test First**: Write tests for new functionality
3. **Backward Compatibility**: Maintain compatibility with existing APIs unless explicitly changing them
4. **Performance**: Consider performance implications, especially for database operations
5. **Error Handling**: Use Rust's Result type for error handling; avoid panics in library code

### Pull Requests

1. Run `cargo test` before submitting
2. Run `cargo clippy` and address warnings
3. Run `cargo fmt` to ensure consistent formatting
4. Update relevant documentation in `design/` if architecture changes
5. Reference related issues in commit messages

### Dependencies

-   Minimize new dependencies
-   When adding dependencies, prefer well-maintained, popular crates
-   Check licensing compatibility (must be compatible with LGPL-3.0-only)

## Common Tasks

### Adding a New Feature

1. Review relevant design documentation
2. Write tests for the new feature
3. Implement the feature in the appropriate module
4. Update documentation if needed
5. Ensure all tests pass

### Fixing a Bug

1. Write a test that reproduces the bug
2. Fix the bug with minimal changes
3. Verify the test now passes
4. Ensure no regressions in other tests

### Refactoring

1. Ensure full test coverage of the area being refactored
2. Make incremental changes
3. Run tests after each change
4. Maintain the same external API unless intentionally changing it

## Reporting Issues

-   Bug reports and enhancement requests: [GitHub Issues](https://github.com/jeleniel/epiloglite/issues)
-   Questions and discussions: [GitHub Discussions](https://github.com/jeleniel/epiloglite/discussions)
-   Security vulnerabilities: Use GitHub's private security vulnerability reporting

## Warnings to Address

The project currently has some naming convention warnings (e.g., enum variants should be UpperCamelCase). When working on files with these warnings, consider fixing them as part of your changes if appropriate.

## Additional Resources

-   [Online Documentation](https://github.com/jeleniel/epiloglite/wiki)
-   [Repository](https://github.com/jeleniel/epiloglite)
