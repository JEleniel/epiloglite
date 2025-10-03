# Core Database Engine

## Overview

EpilogLite must provide a complete, SQLite-compatible database engine implemented entirely in safe Rust. The engine must support fundamental database operations including CREATE, INSERT, SELECT, UPDATE, and DELETE statements with full ACID transaction support.

## User Story

As a Rust developer, I need a pure-Rust database engine that is memory-safe and SQLite-compatible so that I can build reliable applications without depending on C libraries while maintaining compatibility with existing SQLite databases.

## Features

### 1. SQL Query Processing
- Full SQL tokenization and parsing
- Support for basic SQL statements (CREATE TABLE, INSERT, SELECT, UPDATE, DELETE)
- Transaction support (BEGIN, COMMIT, ROLLBACK)
- Type-safe value handling

**Acceptance Criteria:**
- Tokenizer recognizes 100+ SQL keywords and operators
- Parser correctly generates AST for all supported statements
- All basic SQL operations execute without errors
- Transactions maintain ACID properties

### 2. Storage Engine
- Disk persistence with binary serialization
- In-memory database support
- Page-based storage management
- B-tree data structures for efficient data access

**Acceptance Criteria:**
- Data persists to disk and survives application restart
- In-memory mode (:memory:) works without file I/O
- Page cache effectively manages memory usage
- B-tree operations provide O(log n) access time

### 3. Type System
- Support for INTEGER, TEXT, REAL, BLOB, and BOOLEAN types
- NULL value handling
- Native Rust type mapping (17+ types: i8-i128, u8-u128, f32/f64, String, Vec<u8>, bool)
- Type validation and conversion

**Acceptance Criteria:**
- All supported types serialize/deserialize correctly
- NULL values handled appropriately
- Type mismatches generate clear error messages
- Rust native types map correctly to SQL types

### 4. Transaction Management
- ACID compliance
- BEGIN/COMMIT/ROLLBACK support
- Isolation levels
- Rollback on error

**Acceptance Criteria:**
- Transactions are atomic (all-or-nothing)
- Committed changes persist to disk
- Rollback completely undoes uncommitted changes
- Concurrent transactions maintain consistency

### 5. Error Handling
- Comprehensive error types
- Clear, actionable error messages
- No panics in normal operation
- Graceful degradation

**Acceptance Criteria:**
- All operations return Result types
- Error messages clearly indicate the problem and location
- No unwrap() or expect() in production code
- All error conditions are tested
