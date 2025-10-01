# Changelog

All notable changes to EpilogLite will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added - Phase 8: Query Builder Pattern
- Query builder fluent interface for type-safe SQL construction
- SelectBuilder for SELECT queries with WHERE, ORDER BY, LIMIT, OFFSET
- InsertBuilder for INSERT statements with column specification
- UpdateBuilder for UPDATE statements with SET clauses
- DeleteBuilder for DELETE statements with conditions
- CreateTableBuilder for CREATE TABLE with columns and constraints
- QueryBuilder trait for common interface
- Database::execute_builder() method
- query_builder_example.rs demonstration
- 8 new tests for query builders

### Added - Phase 7: Disk Persistence
- Full disk persistence - data saves to and loads from files
- Automatic save on data modifications (INSERT, UPDATE, DELETE, CREATE)
- Binary serialization using bincode
- StorageManager integration with Pager for disk I/O
- Dirty tracking for efficient saves
- Load from disk on database open
- test_disk_persistence integration test
- SerDe support for Table and ColumnDefinition
- Dual mode support (in-memory and file-based)

### Added - Phase 6: In-Memory Storage
- StorageManager for table lifecycle management
- Row-based data storage with actual persistence
- Table operations: create, insert, select_all, update, delete
- Column validation and constraint checking
- Multiple table support
- Data actually stores and retrieves (not stubs)
- 6 storage tests

### Added - Phase 5: SQL Execution Pipeline
- Full SQL tokenizer with 100+ keywords
- Functional SQL parser for all basic statements
- Parser extracts real identifier names from source
- Integrated SQL processor coordinating execution
- ExecutionResult types (Select, RowsAffected, Success)
- Working SQL execution for CREATE, INSERT, SELECT, UPDATE, DELETE
- Transaction statement support (BEGIN, COMMIT, ROLLBACK)
- 19 new tests for parser and processor

### Added - Phase 4: Enhanced Persistence
- Pager with actual disk I/O support
- Page read/write operations with dirty tracking
- LRU-like cache eviction policy
- Page allocation system
- File-backed pager support
- 3 new pager tests

### Added - Phase 3: OS Abstraction Layer
- DefaultFile implementation with file I/O (open, read, write, sync, truncate)
- VirtualFileSystem trait and DefaultVfs implementation
- File locking structure
- Time and random number generation utilities
- basic_usage.rs example
- 4 OS layer tests

### Added - Phase 2: Enhanced Types
- ValueType enum with 17+ Rust native types
- ColumnType definitions with SQL name mappings
- Type checking methods (is_numeric, is_integer, is_float, is_text, is_blob)
- Database header parsing (SQLite 3 & EPLite formats)
- 15 type and header tests

### Added - Phase 1: Core Foundation
- Error handling with comprehensive Error enum
- Result type for ergonomic error handling
- Database format constants (TextEncoding, SchemaFormat)
- Module structure (command, persistence, os, utility)
- 33 foundation tests

## [0.1.0] - 2024-12-XX

### Features

#### Core Database Operations
- CREATE TABLE with column definitions and constraints
- INSERT INTO with values
- SELECT * FROM tables (data retrieval)
- UPDATE with SET clauses
- DELETE FROM tables
- BEGIN/COMMIT/ROLLBACK transactions
- Multiple tables in same database

#### Persistence
- Disk-based database files
- In-memory databases (`:memory:`)
- Automatic save/load
- Page-based storage with cache
- B-tree structure (framework)

#### Query Builder
- Fluent interface for all SQL operations
- Type-safe query construction
- Method chaining
- All statement types supported

#### Type System
- 17+ Rust native types supported
- SQL type mappings (INTEGER, TEXT, REAL, BLOB, BOOLEAN)
- NULL value support
- Type validation

#### Safety & Quality
- 100% safe Rust (no unsafe code)
- 93 tests passing (88 unit + 5 integration)
- Comprehensive error handling
- Well-documented APIs

### Architecture
- Modular design with clear separation
- Command module (tokenizer, parser, processor, VM)
- Persistence module (pager, btree, header)
- Storage module (tables, rows)
- Query builder module
- OS abstraction layer (VFS, file I/O)
- Type system and utilities

### Examples
- basic_usage.rs - Complete workflow
- query_builder_example.rs - Builder patterns

### Documentation
- README.md - Complete feature overview
- STATUS.md - Detailed module status
- CHANGELOG.md - Version history
- CONTRIBUTING.md - Contributor guide
- ARCHITECTURE.md - System design
- FILEFORMAT.md - Database format
- VIRTUALMACHINE.md - VM specification

### Known Limitations
- WHERE clause evaluation not fully implemented
- JOIN operations not implemented
- Aggregate functions not implemented
- No indexing yet
- No query optimization yet
- Single-threaded

## Future Releases

### [0.2.0] - Planned
- WHERE clause filtering
- JOIN operations
- Aggregate functions (COUNT, SUM, AVG, MIN, MAX)
- ORDER BY / GROUP BY execution
- Improved B-tree operations

### [0.3.0] - Planned
- Index support
- Query optimizer
- Performance improvements
- Concurrent access support

### [0.4.0] - Planned
- Unicode 16 full support
- Graph data structures
- Role-based permissions
- Lightweight ORM

### [1.0.0] - Planned
- SQLite C API compatibility
- REST API server mode
- GraphQL support
- Production-ready stability
- 90%+ test coverage
- Comprehensive benchmarks

## Development Notes

### Testing
All phases include comprehensive tests:
- Unit tests for each module
- Integration tests for workflows
- Example programs for demonstrations

### Code Quality
- Follows Rust Style Guide
- Uses tabs for indentation
- Comprehensive documentation
- Zero warnings policy (in progress)

### Dependencies
- serde - Serialization
- bincode - Binary serialization
- flagset - Flag sets
- logos - Lexer generation
- thiserror - Error handling
- tokio - Async runtime (optional)

### Safety
- unsafe_code = "forbid"
- No panics in production code
- Comprehensive error handling
- Memory safe by design
