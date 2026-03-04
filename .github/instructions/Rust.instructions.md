---
applyTo: '*.rs'
---

# Rust Coding Guide

This document defines formatting and style conventions for all Rust source code.

## Formatting Rules

- **Edition (Required)** Always use the Rust 2024 or newer edition style guide, unless the existing code uses an older version, in which case notify the user and get permission before using the older style. You MUST NOT use `mod.rs` files.
- **Organization** Organize code into logical modules that conform to the _single responsibility_ principle and the Rust 2024 style. Minimize top level `*.rs` files using modules.
- **Minimize Lines per File** Aim for a maximum of approximately 200 lines per file. Modules SHOULD only contain a single primary structure and supporting elements _for that module only_. Shared supporting elements MUST be placed in separate files.
- **Indentation (Accessibility)** Prefer hard tabs for indentation, unless the `rustfmt.toml` specifies otherwise. `cargo fmt` is the source of truth; if this repo includes a `rustfmt.toml`, it MUST be treated as authoritative. If creating or modifying Rust code and the `rustfmt.toml` is missing, stop and notify the user.
- **Line Endings:** You MUST use POSIX-style newlines (`\n`).
- **Comment Formatting:**
    * You MUST limit comments to 100 characters per line and wrap for readability - break at word boundaries and avoid hyphenation.
    * Format code in comments. Use `//!` for module/crate docs and `///` for item docs.
    * Ensure that existing comments are kept up to date and accurate to what they describe.
    * You MUST conform to the [rustdoc book](https://doc.rust-lang.org/rustdoc/).
- **Imports:** Group imports by standard, external, and crate. Use crate-level granularity and sort per the best style guide. Place `mod` commands first in the file, after the module documentation, followed by a blank line and then `use` commands.
- **Hexadecimal Literals:** Use uppercase for hex literals. Other uses of hexadecimal should be consistent with idiomatic styles.
- **Wildcards:** Use the underscore wildcard (`_`) for single items, and the rest pattern (`..`) for multiple items.
- **Field Initialization:** Use field initialization shorthand where possible (e.g., `property,` instead of `property: property,`).
- Follow the [2024 Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/index.html) as the baseline for all rules not covered here.

## Coding Rules

- You MUST follow these "Coding Rules" for all code that you write or modify only. Existing code may have been written by other agents of users and you MUST NOT alter it simply for conformance.
- You MUST ensure that all code you write passes all tests and lints.
- You SHOULD use `mcp_cargo-mcp_*` for Rust cargo operations when available. If it is not available, use the standard `cargo` CLI.
- Use `cargo` tools (`fmt`, `clippy`, etc.) to enforce code quality. You MUST fix all warnings and errors in the code you write.
- Unimplemented code paths MUST fail fast and be explicit (prefer `todo!();`, `unimplemented!();`, or similar). Unused variables must be prefixed with an underscore (`_`). Both unimplemented code and unused variables MUST include a comment explaining the intended future use or implementation.
- You MUST NOT use `unwrap`, `expect`, `panic`, or similar unless explicitly instructed by the user. You MUST use `thiserror` and `anyhow` for error handling:
    * Library functions MUST return a well typed `thiserror::Error`.
    * Executables and application boundaries MUST log the errors and exit with an error specific exit code. Prefer `anyhow` for ergonomic context (`anyhow::Context`) and `anyhow::Result`.
    * Use the `?` shorthand and `#[from]` construct to map errors into `thiserror` enums.
    * All errors MUST be handled. The code must make every effort to avoid crashing and return an appropriate type of error or exit value.
    * You MUST NOT use `#[allow(...)]` constructs, or otherwise disable code checks. Fix the error, not the reporting.
- You MUST include documentation comments for all modules and public functions that you create.
- You SHOULD NOT use `unsafe` code; unless a specific call or library requires it due to this being on an embedded platform.
- Include a single reverse-DNS app ID in `Cargo.toml` as `app_id = "{Application ID}"`.

## Libraries

The following libraries are approved for use; always use the latest stable release. Sublibraries are those that start with the name of the parent or are designed to work as children of the parent:

- `axum` (and sublibraries), `tower` (and sublibraries), and `hyper` (and sublibraries) for web server functions - always configure them to use TLS 1.3 and be well hardened unless instructed otherwise.
- `reqwest` for making HTTP calls, and `urlencoding` and `url` for URL handling
- `chrono` for time and date handling - use RFC 3339 with millisecond resolution when converting to strings.
- `clap` for CLI interfaces, and `config` for configuration file handling. CLI parameters SHOULD override configuration files.
- `dirs` (preferred) or `directories` to identify folders for configuration, data, and caching. A subfolder with the package name from Cargo SHOULD be created within those folders.
    * Applications meant to run as services, such as web servers, should also look for configuration in the OS defined shared and user configuration folders, such as `/etc/{package_name}` on Linux. Treat the `app_id` prefix as the "vendor" on Windows, and as the application specific path (sandbox) on Apple OSes.
- `r2d2`, `r2d2_sqlite`, and `rusqlite` for SQLite; use the `bundled` feature of `rusqlite` to ensure that SQLite libraries are available.
- `rustls` (and `rustls` sublibraries), and `openssl` for TLS/SSL.
- `serde` (and sublibraries), `serde_json`, `serde-binary-adv`, `lowlevel-types`, and other `serde` libraries for serialization and deserialization.
- `log`, and either `fern` (preferred) or `tracing` (and sublibraries) for logging
    * Configure logging to write `TRACE`, `DEBUG`, `INFO`, and `WARN` to `stdout` and `ERROR` to stderr
    * For services, include an option to log to rotated log files and/or to the system logging facility (e.g., syslog, Event Log)
- `ctrlc`, `uuid`, `base64`, `hex`, `num-traits`, `regex`, and `unicode-normalization` for their utility functions
- `anyhow`, and `thiserror` for error handling
- `ollama-rs` for Ollama access
- `tokio` and sublibraries for asynchronous code

## Acceptance Criteria

- All `cargo` check tools (`clippy`, `fmt`, `check`, `test`) pass.

## References

- See [model.rs](rust_example/model.rs) and [model/card.rs](rust_example/model/card.rs) for examples of good code.
