# EpilogLite

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)](https://sqlite.org/)

EpilogLite is a pure Rust implementation of SQLite, designed for safety, reliability, and performance. Built with **100% safe Rust** (no `unsafe` code), it provides a drop-in compatible alternative to SQLite with modern Rust idioms.

## ✨ Features

### Currently Implemented

#### Core Database Operations
- ✅ **Full SQL Support** - CREATE TABLE, INSERT, SELECT, UPDATE, DELETE
- ✅ **Disk Persistence** - Data automatically saves to and loads from files
- ✅ **In-Memory Databases** - Fast `:memory:` mode for temporary data
- ✅ **Transactions** - BEGIN, COMMIT, ROLLBACK, SAVEPOINT, RELEASE support
- ✅ **Write-Ahead Logging (WAL)** - Concurrent readers, improved crash recovery
- ✅ **Multiple Tables** - Create and manage multiple tables simultaneously

#### Query Builder Pattern
- ✅ **Fluent Interface** - Type-safe query construction
- ✅ **SelectBuilder** - Build SELECT queries with WHERE, ORDER BY, LIMIT
- ✅ **InsertBuilder** - Build INSERT statements with column specification
- ✅ **UpdateBuilder** - Build UPDATE statements with SET clauses
- ✅ **DeleteBuilder** - Build DELETE statements with conditions
- ✅ **CreateTableBuilder** - Build CREATE TABLE with columns and constraints

#### Type System
- ✅ **17+ Native Rust Types** - Bool, I8-I128, U8-U128, F32/F64, String, Vec<u8>
- ✅ **SQL Type Mapping** - INTEGER, TEXT, REAL, BLOB, BOOLEAN
- ✅ **NULL Support** - Proper NULL value handling
- ✅ **Type Checking** - Built-in type validation

#### Architecture
- ✅ **100% Safe Rust** - No unsafe code blocks
- ✅ **Modular Design** - Clean separation of concerns
- ✅ **Error Handling** - Comprehensive Result types
- ✅ **Test Coverage** - 196 tests (165 unit + 18 adversarial + 11 integration + 9 WAL integration)
- ✅ **Test Coverage** - 148 tests including async I/O operations
- ✅ **Security Tested** - SQL injection resistance, malformed input handling
- ✅ **Idiomatic Rust** - Clippy-approved, modern patterns
- ✅ **Type Safety** - ColumnType enum eliminates hardcoded strings
- ✅ **Async I/O** - Non-blocking file operations with Tokio integration

### In Progress
- 🚧 WHERE clause filtering
- 🚧 JOIN operations
- 🚧 Aggregate functions (COUNT, SUM, AVG, MIN, MAX)
- 🚧 ORDER BY and GROUP BY implementation
- 🚧 Index support

#### Platform Support
- ✅ **No-std Compatible** - Works without standard library for embedded systems
- ✅ **In-memory Mode** - Available in no-std environments
- ✅ **Custom Allocators** - Bring your own allocator support

### Planned
- 📋 Unicode 16 support
- 📋 Graph data structures
- 📋 Role-based permissions
- 📋 Lightweight ORM
- 📋 REST/GraphQL API
- 📋 SQLite C API compatibility
- 📋 Embedded VFS for flash storage

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
epiloglite = "0.1"
```

### Basic Usage

```rust
use epiloglite::{Database, ExecutionResult, Result};

fn main() -> Result<()> {
    // Open or create a database file
    let mut db = Database::open("mydata.db")?;
    
    // Create a table
    db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)")?;
    
    // Insert data
    db.execute("INSERT INTO users VALUES (1, 'Alice', 30)")?;
    db.execute("INSERT INTO users VALUES (2, 'Bob', 25)")?;
    
    // Query data
    let result = db.execute("SELECT * FROM users")?;
    match result {
        ExecutionResult::Select { rows, columns } => {
            println!("Found {} rows", rows.len());
            for row in rows {
                println!("{:?}", row);
            }
        }
        _ => {}
    }
    
    // Update data
    db.execute("UPDATE users SET age = 31 WHERE id = 1")?;
    
    // Delete data
    db.execute("DELETE FROM users WHERE id = 2")?;
    
    // Close database (auto-saves)
    db.close()?;
    
    Ok(())
}
```

### Query Builder Pattern

```rust
use epiloglite::{Database, SelectBuilder, InsertBuilder, UpdateBuilder, Result};

fn main() -> Result<()> {
    let mut db = Database::open("mydata.db")?;
    
    // Create table with builder
    let sql = CreateTableBuilder::new()
        .table("products")
        .column("id", "INTEGER", &["PRIMARY KEY"])
        .simple_column("name", "TEXT")
        .simple_column("price", "INTEGER")
        .build()?;
    db.execute(&sql)?;
    
    // Insert with builder
    let sql = InsertBuilder::new()
        .into("products")
        .columns(&["id", "name", "price"])
        .values(&["1", "'Widget'", "100"])
        .build()?;
    db.execute(&sql)?;
    
    // Query with builder
    let sql = SelectBuilder::new()
        .select_all()
        .from("products")
        .where_clause("price > 50")
        .order_by("name")
        .limit(10)
        .build()?;
    let result = db.execute(&sql)?;
    
    // Update with builder
    let sql = UpdateBuilder::new()
        .table("products")
        .set("price", "120")
        .where_clause("id = 1")
        .build()?;
    db.execute(&sql)?;
    
    db.close()?;
    Ok(())
}
```

### In-Memory Database

```rust
use epiloglite::{Database, Result};

fn main() -> Result<()> {
    // Create in-memory database (faster, no disk I/O)
    let mut db = Database::open(":memory:")?;
    
    // Use like a regular database
    db.execute("CREATE TABLE temp_data (id INTEGER, value TEXT)")?;
    db.execute("INSERT INTO temp_data VALUES (1, 'test')")?;
    
    // Data is lost when database is closed
    db.close()?;
    
    Ok(())
}
```

### No-std / Embedded Usage

EpilogLite can run without the standard library for embedded systems:

```toml
[dependencies]
epiloglite = { version = "0.1", default-features = false, features = ["no-std"] }
```

```rust
#![no_std]

extern crate alloc;

fn main() -> Result<(), epiloglite::Error> {
    // Create an in-memory database (no file I/O in no-std)
    let mut db = epiloglite::Database::new()?;
    
    db.execute("CREATE TABLE sensors (id INTEGER, value INTEGER)")?;
    db.execute("INSERT INTO sensors VALUES (1, 23)")?;
    
    let result = db.execute("SELECT * FROM sensors")?;
    
    Ok(())
}
```

See `docs/NO_STD.md` for detailed documentation and `examples/embedded/` for more examples.

### Async I/O Support

EpilogLite supports asynchronous I/O operations for non-blocking database access:

```toml
[dependencies]
epiloglite = { version = "0.1", features = ["async"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use epiloglite::{AsyncFile, async_file::AsyncDefaultFile};
use epiloglite::SynchronizationType;
use flagset::FlagSet;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Open file asynchronously
    let mut file = AsyncDefaultFile::open("/tmp/test.db", true, true, true).await?;
    
    // Write data asynchronously (non-blocking)
    let data = b"Hello, async world!";
    file.write(data, 0).await?;
    
    // Sync to disk asynchronously
    file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull)).await?;
    
    // Read data back asynchronously
    let read_data = file.read(0).await?;
    println!("Read {} bytes", read_data.len());
    
    Ok(())
}
```

**Key Features:**
- Non-blocking file operations
- Tokio integration
- Backpressure control for concurrent operations
- Performance benchmarking utilities
- Async VFS trait for custom implementations

See `docs/design/ASYNC_IO.md` for detailed documentation and `examples/async_io.rs` for more examples.

## 📚 Examples

The `examples/` directory contains several examples:

- `basic_usage.rs` - Complete workflow demonstration
- `query_builder_example.rs` - Query builder pattern examples
- `savepoint_example.rs` - Transaction savepoint operations
- `embedded/no_std_basic.rs` - No-std embedded example

Run examples with:

```bash
cargo run --example basic_usage
cargo run --example query_builder_example
cargo run --example savepoint_example

# No-std build (won't run but demonstrates compilation)
cargo build --example no_std_basic --no-default-features --features no-std
```

### WAL Mode Example

```rust
use epiloglite::{Database, eplite::persistence::wal::CheckpointMode};

// Open database with Write-Ahead Logging
let mut db = Database::open_with_wal("mydata.db")?;

// Begin transaction
db.begin_transaction()?;

// Perform operations
db.execute("CREATE TABLE items (id INTEGER, name TEXT)")?;
db.execute("INSERT INTO items VALUES (1, 'Item 1')")?;
db.execute("INSERT INTO items VALUES (2, 'Item 2')")?;

// Commit transaction
db.commit_transaction()?;

// Perform checkpoint to transfer WAL to main database
db.checkpoint(CheckpointMode::Full)?;
```

## 🏗️ Architecture

EpilogLite follows a modular architecture:

```
eplite/
├── command/         # SQL parsing and execution
│   ├── tokenizer    # Lexical analysis
│   ├── parser       # Syntax analysis
│   ├── processor    # Query execution
│   └── virtual_machine # Bytecode VM
├── persistence/     # Storage engine
│   ├── pager        # Page cache management
│   ├── btree        # B-tree implementation
│   ├── header       # Database header
│   └── wal          # Write-Ahead Logging
├── storage/         # Table and row management
├── query_builder/   # Fluent query interface
├── os/              # OS abstraction layer
└── types/           # Type system
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test category
cargo test integration
cargo test query_builder

# Run with output
cargo test -- --nocapture
```

Current test coverage: **93 tests passing** (88 unit + 5 integration)

## 📖 Documentation

- [Architecture](design/ARCHITECTURE.md) - System architecture overview
- [File Format](design/FILEFORMAT.md) - Database file format specification
- [Virtual Machine](design/VIRTUALMACHINE.md) - Bytecode execution engine
- [WAL Implementation](docs/WAL_IMPLEMENTATION.md) - Write-Ahead Logging guide
- [C/C++ Interface](design/C-CPP-Interface.md) - C API design (planned)
- [Status](STATUS.md) - Current implementation status
- [Contributing](CONTRIBUTING.md) - Contribution guidelines
- [Changelog](CHANGELOG.md) - Version history

## 🔒 Safety & Security

- **100% Safe Rust** - No `unsafe` blocks anywhere
- **Comprehensive Error Handling** - All errors properly handled
- **No Panics** - Graceful error returns
- **Memory Safe** - Rust's ownership system prevents common bugs
- **Thread Safe** - Designed for concurrent access

## 🎯 Goals

EpilogLite aims to:

1. Provide a **safe** alternative to SQLite using pure Rust
2. Maintain **SQLite 3 compatibility** for existing databases
3. Offer **modern Rust idioms** (builders, async, etc.)
4. Support **all major platforms** (Windows, Linux, macOS, mobile, embedded)
5. Achieve **high performance** without sacrificing safety

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

EpilogLite is licensed under the LGPL-3.0-only license.

Copyright (C) 2024 Tony M. Bishop

## 🙏 Acknowledgments

- SQLite project for the original design and inspiration
- Rust community for excellent tooling and libraries

## 📬 Contact

For questions, issues, or contributions, please use GitHub Issues.
