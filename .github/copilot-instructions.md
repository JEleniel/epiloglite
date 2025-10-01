# GitHub Copilot Instructions for EpilogLite

## Project Overview

EpilogLite is a Rust implementation of the SQLite database library. It's a small, fast, self-contained, high-reliability, full-featured SQL database engine that aims to be compatible with the SQLite file format while adding new, updated, and extended capabilities.

**Key Facts:**
- Written in pure Rust (2021 edition)
- No dependencies on the SQLite C library - clean reimplementation
- Targets SQLite 3.0 API compatibility
- Early development stage (alpha, version 0.x.x)
- Licensed under GNU LGPL 3.0 only

## Architecture

EpilogLite compiles SQL text into bytecode and runs it using a virtual machine. The architecture is modular:

### Main Modules
- **epiloglite**: Public async interface for the database engine
- **sqlite**: C ABI compatible functions (drop-in replacement for SQLite C library)
- **command**: SQL processing and execution
  - `processor`: Command processing
  - `compiler`: SQL compilation (tokenizer, parser, code_generator)
  - `virtual_machine`: Bytecode execution
- **persistence**: Data storage
  - `btree`: B-tree implementation (SQLite-compatible file format)
  - `pager`: Page cache and transaction management
- **utility**: Utility functions
- **os**: Operating system interfaces

See [design/ARCHITECTURE.md](../design/ARCHITECTURE.md) for detailed architecture diagrams.

## Build and Test

### Building
```bash
# Development build
cargo build

# Release build
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

### Linting
The project uses standard Rust tooling. Run:
```bash
cargo clippy
cargo fmt --check
```

## Code Guidelines

### Safety and Security
- **Unsafe code is FORBIDDEN** - The project has `unsafe_code = "forbid"` in lints
- Never commit secrets or credentials to source code
- Use safe Rust idioms and patterns only

### Code Style
- Follow standard Rust naming conventions:
  - Types: `UpperCamelCase`
  - Functions/variables: `snake_case`
  - Constants: `SCREAMING_SNAKE_CASE`
- Use idiomatic Rust patterns
- Prefer immutability where possible
- Keep modules focused and cohesive

### Documentation
- Use Rust doc comments (`///` and `//!`)
- Follow RFC 2119 keywords (MUST, SHOULD, MAY) as described in [design/00_Frontmtter.md](../design/00_Frontmtter.md)
- All data elements conform to Rust data types
- Use binary prefixes (Ki, Mi, Gi) for byte multiples
- Reference the extensive design documentation in the `design/` directory

### Testing
- Implement tests in the same file as components (standard Rust approach)
- Write unit tests for new functionality
- Consider edge cases and error conditions

## Working with SQL

EpilogLite implements SQL syntax compatible with SQLite. Refer to:
- [design/sql_syntax/Index.md](../design/sql_syntax/Index.md) for SQL syntax reference
- [design/QUERYPLANNER.md](../design/QUERYPLANNER.md) for query optimization
- [design/VIRTUALMACHINE.md](../design/VIRTUALMACHINE.md) for bytecode operations

### Bytecode Instructions
The VM uses instructions with:
- Opcode
- P1, P2, P3 (32-bit signed integers, often register/cursor references)
- P4 (various types: integer, float, string, blob, function pointer)
- P5 (16-bit unsigned flags)

## Dependencies

Keep external dependencies minimal. Currently using:
- `flagset` (v0.4.6) - Flag set handling with serde support
- `logos` (v0.15.0) - Lexical analysis

Only add new dependencies if absolutely necessary.

## File Format Compatibility

EpilogLite strives to maintain compatibility with the SQLite file format:
- B-tree structure must remain compatible
- Default page size: 4096 bytes (configurable: 512 to 65536 bytes, power of 2)
- Refer to design documentation when making changes to persistence layer

## Documentation Locations

- **Architecture**: `design/ARCHITECTURE.md`
- **File Format**: `design/FILEFORMAT.md`
- **Virtual Machine**: `design/VIRTUALMACHINE.md`
- **Transactions**: `design/TRANSACTIONS.md`
- **Query Planner**: `design/QUERYPLANNER.md`
- **SQL Syntax**: `design/sql_syntax/`
- **C/C++ Interface**: `design/C-CPP-Interface.md`

## Contributing

- Bug reports: [GitHub Issues](https://github.com/jeleniel/epiloglite/issues)
- Discussions: [GitHub Discussions](https://github.com/jeleniel/epiloglite/discussions)
- Security: Use [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability)

## When Making Changes

1. **Understand the component**: Review relevant design documentation first
2. **Check compatibility**: Ensure changes maintain SQLite file format compatibility
3. **Follow Rust best practices**: Use safe, idiomatic Rust
4. **Test thoroughly**: Add tests for new functionality
5. **Document**: Update design docs if architecture changes
6. **No unsafe code**: Remember the `forbid(unsafe_code)` lint
7. **Keep it minimal**: This is a database engine - changes should be well-justified

## Common Patterns

- Functions in the `epiloglite` module are generally asynchronous
- Use Rust's error handling (`Result<T, E>`) consistently
- Leverage the type system for safety
- Reference existing code for style consistency
