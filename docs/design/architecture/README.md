# EpilogLite Architecture Documentation

This directory contains comprehensive architecture documentation for EpilogLite, a pure Rust implementation of SQLite.

## Documentation Structure

### Requirements Documents

Functional requirements organized by feature area:

- **[Requirement_1.md](Requirement_1.md)** - Core Database Engine
  - SQL query processing
  - Storage engine
  - Type system
  - Transaction management
  - Error handling

- **[Requirement_2.md](Requirement_2.md)** - Advanced SQL Features
  - WHERE clause filtering
  - JOIN operations
  - Aggregate functions
  - Sorting and grouping
  - Subqueries

- **[Requirement_3.md](Requirement_3.md)** - Performance and Optimization
  - Indexing system
  - Query optimizer
  - Memory management
  - Disk I/O optimization
  - Concurrency

- **[Requirement_4.md](Requirement_4.md)** - Developer Experience and APIs
  - Query builder pattern
  - Lightweight ORM
  - Async/await support
  - C API compatibility
  - Configuration and logging

- **[Requirement_5.md](Requirement_5.md)** - Server Mode and Network APIs
  - REST API
  - GraphQL API
  - Authentication and authorization
  - TLS/SSL encryption
  - Client libraries

- **[Requirement_6.md](Requirement_6.md)** - Platform Support and Portability
  - Operating system support
  - No-std support
  - Cross-compilation
  - Unicode support
  - Resource constraints

### Non-Functional Requirements

- **[NFRs.md](NFRs.md)** - Non-Functional Requirements
  - Security (memory safety, SQL injection, encryption)
  - Reliability (error handling, data integrity, crash recovery)
  - Performance (query performance, memory usage, scalability)
  - Maintainability (code quality, documentation, modularity)
  - Scalability (concurrent users, data volume, transaction rate)
  - Usability (API design, error messages, examples)
  - Compliance (licensing, standards, accessibility)
  - Portability (platform independence, architecture support)
  - Observability (logging, metrics, profiling)

### Architecture Diagrams

Visual representations of the system architecture:

- **[SystemArchitecture.md](SystemArchitecture.md)** - High-Level Architecture
  - Component architecture diagram
  - Layer descriptions
  - Module interactions

- **[DataFlow.md](DataFlow.md)** - Data Flow Architecture
  - Query execution flow
  - Insert operation flow
  - Transaction flow
  - Page cache operation
  - Module communication
  - Error propagation

- **[DeploymentArchitecture.md](DeploymentArchitecture.md)** - Deployment Scenarios
  - Embedded library mode
  - Standalone server mode
  - C API integration mode
  - Microservice architecture
  - Multi-tier architecture
  - High availability setup (planned)
  - Platform support matrix

## Related Documentation

### Design Documents

Additional design documentation can be found in the parent directory:

- [../ARCHITECTURE.md](../ARCHITECTURE.md) - Original architecture overview
- [../VIRTUALMACHINE.md](../VIRTUALMACHINE.md) - Virtual machine design
- [../QUERYPLANNER.md](../QUERYPLANNER.md) - Query optimizer design
- [../FILEFORMAT.md](../FILEFORMAT.md) - Database file format specification
- [../TRANSACTIONS.md](../TRANSACTIONS.md) - Transaction implementation
- [../C-CPP-Interface.md](../C-CPP-Interface.md) - C/C++ API interface

### Implementation Documentation

For developers and machine agents:

- [../agents/IMPLEMENTATION_SUMMARY.md](../agents/IMPLEMENTATION_SUMMARY.md) - Current implementation state
- [../agents/TODO.md](../agents/TODO.md) - Phased development plan

### Project Documentation

- [../../../README.md](../../../README.md) - Project overview and quick start
- [../../../STATUS.md](../../../STATUS.md) - Detailed implementation status
- [../../../CONTRIBUTING.md](../../../CONTRIBUTING.md) - Contribution guidelines
- [../../../CHANGELOG.md](../../../CHANGELOG.md) - Version history

## Document Conventions

All architecture documents follow these conventions:

1. **Markdown Format**: GitHub-Flavored Markdown (GFM)
2. **Diagrams**: Mermaid syntax for all diagrams
3. **Structure**: Consistent heading hierarchy
4. **Acceptance Criteria**: Each feature includes clear acceptance criteria
5. **Cross-References**: Links to related documents where relevant

## How to Use This Documentation

### For System Architects

1. Start with [SystemArchitecture.md](SystemArchitecture.md) for the big picture
2. Review requirement documents (Requirement_1.md through Requirement_6.md)
3. Study [NFRs.md](NFRs.md) for quality attributes
4. Examine [DataFlow.md](DataFlow.md) for runtime behavior
5. Review [DeploymentArchitecture.md](DeploymentArchitecture.md) for deployment options

### For Developers

1. Read [../agents/IMPLEMENTATION_SUMMARY.md](../agents/IMPLEMENTATION_SUMMARY.md) for current state
2. Check [../agents/TODO.md](../agents/TODO.md) for work items
3. Review relevant requirement documents for feature details
4. Study [DataFlow.md](DataFlow.md) to understand execution flows
5. Refer to design documents in parent directory for implementation details

### For Machine Agents

1. **ALWAYS** read [../agents/IMPLEMENTATION_SUMMARY.md](../agents/IMPLEMENTATION_SUMMARY.md) first
2. Check [../agents/TODO.md](../agents/TODO.md) for prioritized tasks
3. Review requirement documents for feature specifications
4. Use [NFRs.md](NFRs.md) to ensure quality requirements are met
5. Refer to architecture diagrams to understand system structure

## Maintenance

This documentation should be updated when:

- New features are designed or planned
- Architecture changes significantly
- New deployment modes are added
- Non-functional requirements change
- Implementation reaches significant milestones

Keep documentation synchronized with:

- Source code implementation
- Test coverage
- Public API changes
- Configuration options
- Deployment procedures

## Questions?

For questions about this documentation:

- Review the [CONTRIBUTING.md](../../../CONTRIBUTING.md) guide
- Check [GitHub Discussions](https://github.com/JEleniel/epiloglite/discussions)
- Open an issue on [GitHub Issues](https://github.com/JEleniel/epiloglite/issues)
