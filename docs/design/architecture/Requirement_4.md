# Developer Experience and APIs

## Overview

EpilogLite must provide intuitive, type-safe APIs for Rust developers including a fluent query builder pattern, ORM capabilities, and SQLite C API compatibility for drop-in replacement scenarios.

## User Story

As a developer, I need ergonomic, type-safe APIs with good documentation so that I can quickly integrate EpilogLite into my projects and write maintainable database code with minimal boilerplate.

## Features

### 1. Query Builder Pattern

- Fluent, chainable interface
- SelectBuilder for queries
- InsertBuilder for inserts
- UpdateBuilder for updates
- DeleteBuilder for deletes
- CreateTableBuilder for schema
- Type-safe at compile time

**Acceptance Criteria:**

- All SQL operations have corresponding builders
- Method chaining works intuitively
- Build errors caught at compile time where possible
- Generated SQL is valid and efficient
- Builders support all SQL features

### 2. Lightweight ORM

- Entity trait for domain objects
- Repository pattern for CRUD operations
- Automatic mapping between structs and tables
- Type-safe column access
- Relationship mapping (planned)

**Acceptance Criteria:**

- Derive macro for Entity trait
- CRUD operations work without writing SQL
- Type mismatches caught at compile time
- Relationships (1:1, 1:N, N:M) supported
- Lazy loading for relationships

### 3. Async/Await Support

- Async database operations
- Non-blocking I/O
- Integration with Tokio runtime
- Stream-based result sets
- Concurrent query execution

**Acceptance Criteria:**

- All database operations have async variants
- No blocking in async context
- Proper cancellation support
- Backpressure handling for large result sets
- Compatible with async ecosystem

### 4. C API Compatibility

- SQLite 3 C API functions
- Drop-in replacement capability
- C ABI compatibility
- Feature-gated (cabi feature)
- Existing SQLite applications work unchanged

**Acceptance Criteria:**

- Core sqlite3_* functions implemented
- C applications compile and run without modification
- Behavior matches SQLite for supported features
- Memory management follows C API conventions
- Error codes match SQLite

### 5. Configuration and Logging

- JSON/TOML configuration files
- Environment variable overrides
- Multiple log targets (file, stdout, syslog)
- Colored output for development
- Log levels (ERROR, WARN, INFO, DEBUG, TRACE)
- Per-module log configuration

**Acceptance Criteria:**

- Configuration loads from files and environment
- Logging works with multiple targets simultaneously
- Log output is clear and helpful
- Performance impact of logging is minimal
- Sensitive data not logged
