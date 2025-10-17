# EpilogLite Implementation Plan

This document outlines a staged implementation plan to bring EpilogLite to full feature completion, based on the current design, codebase, and roadmap in the documentation.

---

## Stage 1: Core Engine & Persistence

- [ ] Complete page and collection abstraction (all logical elements as collections)
- [ ] Finalize journal format and atomic commit/recovery logic
- [ ] Robust CRC/integrity checks for all persistent structures
- [ ] Implement and test migration/versioning support
- [ ] Error handling: ensure all errors use idiomatic Rust patterns and are surfaced appropriately
- [ ] Self-tuning and developer-tunable configuration parameters (page size, cache, etc.)
- [ ] In-memory and disk persistence modes
- [ ] Basic SQL parser and execution engine (CREATE, INSERT, SELECT, UPDATE, DELETE)
- [ ] Unit and integration tests for all core features

## Stage 2: SQL Feature Expansion

- [ ] Full ANSI and SQLite SQL support (including WHERE, JOIN, ORDER BY, GROUP BY, aggregates)
- [ ] Query builder API (fluent, type-safe)
- [ ] Index support (creation, maintenance, and query planning)
- [ ] NULL value handling and type validation
- [ ] Vector and advanced type support (Vec<T>, Unicode, etc.)
- [ ] Savepoint, rollback, and nested transaction support
- [ ] Comprehensive error and constraint handling (malformed input, SQL injection resistance)
- [ ] Test coverage for all SQL features

## Stage 3: Advanced Data Models & Extensibility

- [ ] Graph data model support (nodes, edges, hybrid tabular/graph queries)
- [ ] Modular plugin/extension system (async traits, custom storage backends)
- [ ] Lightweight ORM and schema introspection
- [ ] REST/GraphQL API (optional server feature)
- [ ] Embedded VFS for flash/embedded storage
- [ ] No-std/embedded support (with feature flags)
- [ ] SQLite C API compatibility (drop-in replacement)
- [ ] Thread safety and concurrency improvements (multi-writer, page/record locking)
- [ ] Role-based permissions and access control (future)
- [ ] Encryption and tamper detection (future)

## Stage 4: Performance, Testing, and Hardening

- [ ] Performance benchmarking and tuning (page size, cache, journaling)
- [ ] Fault injection and recovery validation
- [ ] Fuzz and property-based testing
- [ ] Documentation: update and expand all user and developer docs
- [ ] Security review and hardening
- [ ] Finalize API and prepare for 1.0 release

---

## Notes

- Stages may overlap; some features (e.g., error handling, testing) are ongoing.
- Features marked "future" are not required for 1.0 but should be designed for extensibility.
- See `README.md` and `docs/` for detailed feature descriptions and design rationale.
