# EpilogLite Source Repository

This repository contains the complete source code for the EpilogLite database engine, including test scripts and examples.

## What is EpilogLite?

EpilogLite is a port and refactoring of SQLite 3 to pure Rust without unsafe code. It provides:

- **Pure Rust Implementation**: 100% safe Rust code with `unsafe_code = "forbid"`
- **SQLite Compatibility**: Backwards compatible with SQLite 3 databases
- **Modern Format**: Improved EPLite format with enhanced features
- **Full Type Support**: Native support for all Rust types (bool, i8-i128, u8-u128, f32/f64, strings, blobs)
- **Cross-Platform**: Supports Windows, Linux, macOS, iOS, Android, and embedded systems
- **No Dependencies on SQLite**: Clean implementation without SQLite library dependencies

## Current Status

**Alpha Development** - Core foundation complete (52 tests passing)

✅ Completed:
- Error handling and result types
- All Rust native type support (17+ types)
- Database file format (SQLite 3 & EPLite)
- File I/O and OS abstraction layer
- Basic module structure

🚧 In Progress:
- SQL tokenizer and parser
- B-tree storage engine
- Virtual machine for bytecode execution
- Transaction support

## Quick Start

### Installation

Add EpilogLite to your `Cargo.toml`:

```toml
[dependencies]
epiloglite = "0.1.0"
```

### Basic Usage

```rust
use epiloglite::{Database, Result};

fn main() -> Result<()> {
    // Open an in-memory database
    let mut db = Database::open(":memory:")?;
    
    // Execute SQL (when implemented)
    // db.execute("CREATE TABLE users (id INTEGER, name TEXT)")?;
    
    // Close the database
    db.close()?;
    Ok(())
}
```

### Running Examples

```shell
cargo run --example basic_usage
```

## Documentation

See the [on-line documentation](https://github.com/jeleniel/epiloglite/wiki) for more information about what EpilogLite is and how it works from a user's perspective.

Design documentation is in the `./design/` folder:
- [Architecture](design/ARCHITECTURE.md) - System architecture and module overview
- [File Format](design/FILEFORMAT.md) - Database file format specification
- [Virtual Machine](design/VIRTUALMACHINE.md) - Bytecode execution engine
- [Query Planner](design/QUERYPLANNER.md) - Query optimization
- [Transactions](design/TRANSACTIONS.md) - Transaction processing

## Testing and Compiling

Since this is a Rust application, the normal 'cargo' commands can be used to test or build the application.

To execute the test suite run:

```shell
cargo test
```

To create a release build run:

```shell
cargo build --release
```

The compiled binaries will be in the 'target' folder after the build completes.

## Features

### Completed
- Safe Rust implementation (no unsafe code)
- Comprehensive error handling
- Support for all Rust native types
- Database file header parsing (SQLite 3 & EPLite formats)
- OS abstraction layer for cross-platform support
- File I/O operations

### Planned
- Full SQL support (SELECT, INSERT, UPDATE, DELETE, CREATE, etc.)
- ACID transactions with rollback
- B-tree storage engine
- Query optimizer and planner
- Role-based permissions
- REST API and GraphQL support
- Lightweight ORM
- C API compatibility layer

## Contributing

Bug reports, enhancement requests, and documentation suggestions can be opened at the [Epilogue Issues](https://github.com/jeleniel/epiloglite/issues) list.

The preferred way to ask questions or make comments about EpilogLite is to visit the [EpilogLite Discussions](https://github.com/jeleniel/epiloglite/discussions).

[Private security vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability) is enabled on this repository.

## Version Control

EpilogLite sources are managed using [GitHub](https://github.com/jeleniel/epiloglite).

## License

The EpilogLite source code is released under the GNU Lesser General Public License 3.0 only. See [LICENSE.md](LICENSE.md) for details.

## Acknowledgments

EpilogLite is inspired by SQLite, one of the most widely deployed database engines in the world. We thank the SQLite team for their excellent work and comprehensive documentation.
