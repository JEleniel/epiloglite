# EpilogLite System Architecture

## Overview

This document provides a high-level overview of the EpilogLite system architecture, showing the major components and their interactions.

## Component Architecture

```mermaid
---
title: EpilogLite High-Level Architecture
---
graph TB
    subgraph "Client Layer"
        A[Rust API]
        B[C API]
        C[Query Builders]
        D[ORM]
    end
    
    subgraph "Command Layer"
        E[Tokenizer]
        F[Parser]
        G[Code Generator]
        H[Virtual Machine]
        I[Processor]
    end
    
    subgraph "Storage Layer"
        J[Storage Manager]
        K[B-tree]
        L[Pager]
    end
    
    subgraph "Infrastructure Layer"
        M[VFS / OS Interface]
        N[Types & Utilities]
        O[Error Handling]
        P[Config & Logging]
    end
    
    subgraph "Server Layer (Optional)"
        Q[REST API]
        R[GraphQL API]
        S[Auth & RBAC]
        T[TLS Layer]
    end
    
    A --> I
    B --> I
    C --> I
    D --> A
    
    I --> E
    E --> F
    F --> G
    G --> H
    H --> J
    
    J --> K
    K --> L
    L --> M
    
    N --> E
    N --> F
    N --> J
    O --> I
    O --> J
    P --> I
    
    Q --> A
    R --> A
    S --> Q
    S --> R
    T --> Q
    T --> R
    
    style A fill:#e1f5ff
    style B fill:#e1f5ff
    style C fill:#e1f5ff
    style D fill:#e1f5ff
    style I fill:#fff4e1
    style J fill:#ffe1e1
    style M fill:#e1ffe1
    style Q fill:#f0e1ff
    style R fill:#f0e1ff
```

## Layer Descriptions

### Client Layer
- **Rust API**: Primary public interface for Rust applications
- **C API**: SQLite-compatible C interface for drop-in replacement
- **Query Builders**: Fluent, type-safe query construction
- **ORM**: Entity/Repository pattern for domain object mapping

### Command Layer
- **Tokenizer**: Lexical analysis of SQL text
- **Parser**: Syntax analysis and AST generation
- **Code Generator**: Bytecode generation and query planning
- **Virtual Machine**: Bytecode execution engine
- **Processor**: Coordinates the entire SQL processing pipeline

### Storage Layer
- **Storage Manager**: Table and row management
- **B-tree**: Indexed data structure for efficient access
- **Pager**: Page cache and disk I/O management

### Infrastructure Layer
- **VFS / OS Interface**: Platform abstraction for file I/O, time, random numbers
- **Types & Utilities**: Type system, value handling, utility functions
- **Error Handling**: Comprehensive error types and Result propagation
- **Config & Logging**: Configuration management and structured logging

### Server Layer (Optional)
- **REST API**: HTTP endpoints for SQL execution
- **GraphQL API**: GraphQL schema and resolvers
- **Auth & RBAC**: JWT authentication and role-based authorization
- **TLS Layer**: TLS 1.3 encryption for network security
