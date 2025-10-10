# EpilogLite™

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)](https://sqlite.org/)

EpilogLite is a pure Rust implementation of SQLite, designed for safety, reliability, and performance. The engine is built with **100% safe Rust** (no `unsafe` code). It also provides a drop-in compatible alternative to SQLite with modern Rust idioms (unsafe code is required for the "C" ABI).

## 🎯 Goals

EpilogLite aims to:

1. Provide a **safe** alternative to SQLite using pure Rust
2. Maintain **SQLite 3 compatibility** for existing databases
3. Offer **modern Rust idioms** (builders, async, etc.)
4. Support **all major platforms** (Windows, Linux, macOS, mobile, embedded)
5. Achieve **high performance** without sacrificing safety

## ✨ Features

### Core Database Operations

- **Full ANSI and SQLite SQL Support** - CREATE TABLE, INSERT, SELECT, UPDATE, DELETE
- **In-Memory and Disk Persistence** - Choose in-memory storage, on disk storage, or any combination of both, including disk backed memory.
- **Transactions** - BEGIN, COMMIT, ROLLBACK, SAVEPOINT, RELEASE support
- **SQLite 3 Compatability** - Read and write SQLite databasesm or use the drop in "C" ABI.

### Query Builders

- **Fluent Interface** - Type-safe query construction
- **SelectBuilder** - Build SELECT queries with WHERE, ORDER BY, LIMIT
- **InsertBuilder** - Build INSERT statements with column specification
- **UpdateBuilder** - Build UPDATE statements with SET clauses
- **DeleteBuilder** - Build DELETE statements with conditions
- **TableBuilder** - Build CREATE and ALTER TABLE with columns and constraints

### Type System

- **17+ Native Rust Types** - Bool, Signed and Unsinged Integers, Floats, Unicode and ASCII Strings, Vec<T> for all numeric types.
- **SQLite Type Mapping** - INTEGER, TEXT, REAL, BLOB, BOOLEAN
- **NULL Support** - Proper NULL value handling
- **Type Checking** - Built-in type validation
- **Vector Support** - Support for Vector indexing and searches
- **Type Safety** - ColumnType enum eliminates hardcoded strings

### Architecture

- **Multi-Modal Data** - Supports both Tabular and Graph data models, or combinations; Rows are nodes and can be linked.
- **100% Safe Idiomatic Rust** - Clippy-approved, modern patterns with no unsafe code blocks outside the "C" ABI
- **Modular Design** - Clean separation of concerns
- **Error Handling** - Comprehensive Result types
- **Security Tested** - SQL injection resistance, malformed input handling
- **Comprehensive Error Handling** - All errors properly handled
- **No Panics** - Graceful error returns
- **Memory Safe** - Rust's ownership system prevents common bugs
- **Thread Safe** - Designed for concurrent access

### Roadmap

- WHERE clause filtering
- JOIN operations
- Aggregate functions (COUNT, SUM, AVG, MIN, MAX)
- ORDER BY and GROUP BY implementation
- Index support
- Unicode 16 support
- Graph data structures
- Role-based permissions
- Lightweight ORM
- REST/GraphQL API
- SQLite C API compatibility
- Embedded VFS for flash storage

## 🚀 Usage

### Feature Flags

- ✅ **Default** - By default the "std" and "async" features are enabled.
- **std** - Enables use of the standard libraries.
- **async** - Enables the "async" traits for plugins and extensions.
- **cabi** - Enables the "C" ABI. Requires "std".
- **server** - Enables a complete, standalone server. Requires "std" and "async".
- **no-std** - Disable the standard library for embedded systems. Not compatible with "server" or "cabi" features.

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
epiloglite = "0.1"
```

#### Basic Usage

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

#### Query Builders

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

#### In-Memory Database

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

### 📚 Examples

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

## 📖 Additional Documentation

- [Architecture](docs/design/ARCHITECTURE.md) - System architecture overview
- [File Format](docs/design/FILEFORMAT.md) - Database file format specification
- [Virtual Machine](docs/design/VIRTUALMACHINE.md) - Bytecode execution engine
- [C/C++ Interface](docs/design/C-CPP-Interface.md) - C API design (planned)

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 Legal

EpilogLite and EpilogServer are copyright &copy; 2025 JEleniel and licensed under either the [MIT License](LICENSE-MIT.md) or the [Apache 2.0 License](LICENSE-Apache.md), at the users discretion.

The EpilogLite name, logo, and other presentational materials are trademarks of JEleniel and may not be used without express, written permission.

## 🙏 Acknowledgments

- SQLite project for the original design and inspiration
- Rust community for excellent tooling and libraries
- Shields.io for the badges

## 📬 Contact

For questions, issues, or contributions, please use GitHub Discussions or Issues.
