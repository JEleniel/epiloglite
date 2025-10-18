# EpilogLite Design Documentation

EpilogLite is a Rust implementation of a database engine inspired by SQLite. This directory contains comprehensive design documentation organized into focused, modular documents.

## Documentation Overview

This design documentation is organized to provide a clear understanding of EpilogLite's architecture, implementation, and future roadmap.

### Core Design Documents

1. **[Overview](01_Overview.md)** - Project overview, goals, and key principles
2. **[Architecture Overview](02_Architecture_Overview.md)** - High-level system architecture and component interaction
3. **[Transaction Model](03_Transaction_Model.md)** - Transaction handling, commit cycles, and consistency guarantees
4. **[Storage and Pages](04_Storage_and_Pages.md)** - Page format, storage layout, and file format specifications
5. **[Journaling and Recovery](05_Journaling_and_Recovery.md)** - Journal-first COW, recovery procedures, and crash safety
6. **[Performance Optimization](06_Performance_Optimization.md)** - Performance characteristics, caching strategies, and optimization techniques
7. **[Future Extensions](07_Future_Extensions.md)** - Planned features, roadmap, and extension points

### Additional Documentation

- **[Async I/O Implementation](ASYNC_IO.md)** - Tokio-based async implementation details
- **[Query Planner](QUERYPLANNER.md)** - Query optimization and execution planning
- **[Virtual Machine](VIRTUALMACHINE.md)** - Bytecode execution engine
- **[C/C++ Interface](C-CPP-Interface.md)** - Foreign function interface for C/C++ compatibility
- **[Stored Procedures](STORED_PROCEDURES.md)** - Stored procedure implementation
- **[SQL Syntax](sql_syntax/)** - Detailed SQL syntax documentation

### Legacy Documents

The following documents contain historical design information and are preserved for reference:

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Original architecture document
- **[TRANSACTIONS.md](TRANSACTIONS.md)** - Original transaction documentation (based on SQLite atomic commit)
- **[FILEFORMAT.md](FILEFORMAT.md)** - Original file format specification

## Key Design Principles

EpilogLite is built on the following core principles:

### Safety First

- **100% Safe Rust**: No `unsafe` code anywhere, including dependencies
- All dependencies must be verified for memory safety
- Robust error handling with comprehensive `Result` types

### Async-First Architecture

- **Tokio-first async API**: Async operations as the primary interface
- Synchronous API deferred to future feature releases
- Non-blocking I/O for high-performance concurrent access

### Transactional Integrity

- **Journal-first COW commit cycle**: Journal → COW → Commit → mark Journal
- Recovery is deterministic from Journal
- No Write-Ahead Log (WAL) in v1, no traditional checkpoints
- ACID guarantees through proven transactional mechanisms

### Storage Architecture

- **Identical page format**: All pages use `page_id`, `container_id`, `crc`, `slot_index`, `Vec<u8>` payload
- Page size is auto-detected at database creation
- Page cache self-tunes for optimal performance
- SQLite-compatible file format where possible

### Deployment Model

- **Single-connection model**: Multi-connection support out of scope for v1
- Simplified concurrency model for initial release
- Clear upgrade path for future multi-connection support

## Development Phases

The project follows a phased development approach with clear milestones:

- **Phase I: Engine MVP** - Core database engine functionality
- **Phase II: Robustness & Performance** - Production-ready reliability and optimization
- **Phase III: Cross-platform Testing** - Platform compatibility and testing
- **Phase IV: Query API & Documentation** - Complete API and comprehensive documentation

Each design document includes phase-specific Kanban boards to track implementation progress.

## Future Features

The following features are documented but out of scope for v1:

- End-to-end encryption support
- Access Control Lists (ACLs)
- `no_std` support for embedded systems
- Optional memory-mapped I/O (mmap)
- Multi-connection/multi-threaded access
- Synchronous API alongside async

## Documentation Conventions

All design documents follow these conventions:

- **RFC 2119 Keywords**: "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" are interpreted as defined in [RFC 2119](https://www.rfc-editor.org/info/rfc2119)
- **Data Types**: All data elements conform to [Rust Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- **Diagrams**: Mermaid diagrams are used for visual representations with properly quoted node labels
- **Code Examples**: Rust code snippets follow the project's style guidelines

## Contributing to Design Documents

When contributing to design documentation:

1. Ensure all Mermaid diagram node labels containing spaces or punctuation are quoted
2. Use Kanban-style task lists with GitHub checkboxes (`- [ ]` for pending, `- [x]` for complete)
3. Keep content modular and focused on a single topic per document
4. Cross-reference related documents using relative links
5. Follow the existing structure and formatting conventions

## License

EpilogLite is licensed under the GNU Lesser General Public License 3.0 only (LGPL-3.0-only).

## Additional Resources

- [Project Repository](https://github.com/jeleniel/epiloglite)
- [Contributing Guidelines](../../CONTRIBUTING.md)
- [Code of Conduct](../../CODE_OF_CONDUCT.md)
- [Security Policy](../../SECURITY.md)
