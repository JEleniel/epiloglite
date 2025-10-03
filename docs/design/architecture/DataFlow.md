# EpilogLite Data Flow Architecture

## Overview

This document illustrates the flow of data through the EpilogLite system for various operations.

## Query Execution Flow

```mermaid
---
title: SELECT Query Execution Flow
---
sequenceDiagram
    participant Client
    participant Database
    participant Processor
    participant Tokenizer
    participant Parser
    participant StorageManager
    participant Pager
    participant VFS
    
    Client->>Database: execute("SELECT * FROM users")
    Database->>Processor: process_sql(sql)
    Processor->>Tokenizer: tokenize(sql)
    Tokenizer->>Processor: tokens
    Processor->>Parser: parse(tokens)
    Parser->>Processor: AST
    Processor->>StorageManager: select(table, columns)
    StorageManager->>Pager: read_pages()
    Pager->>VFS: file.read()
    VFS-->>Pager: data
    Pager-->>StorageManager: pages
    StorageManager-->>Processor: rows
    Processor-->>Database: ExecutionResult::Select
    Database-->>Client: Result<ExecutionResult>
```

## Insert Operation Flow

```mermaid
---
title: INSERT Operation Flow
---
sequenceDiagram
    participant Client
    participant Database
    participant Processor
    participant Parser
    participant StorageManager
    participant Pager
    participant VFS
    
    Client->>Database: execute("INSERT INTO users VALUES (...)")
    Database->>Processor: process_sql(sql)
    Processor->>Parser: parse(tokens)
    Parser->>Processor: AST
    Processor->>StorageManager: insert(table, row)
    StorageManager->>StorageManager: validate_row()
    StorageManager->>Pager: mark_dirty(page_id)
    StorageManager-->>Processor: rows_affected
    Processor->>Database: ExecutionResult::RowsAffected
    Database->>Processor: flush()
    Processor->>StorageManager: flush()
    StorageManager->>Pager: flush_dirty_pages()
    Pager->>VFS: file.write()
    VFS->>VFS: sync()
    VFS-->>Pager: Ok(())
    Pager-->>StorageManager: Ok(())
    Database-->>Client: Result<ExecutionResult>
```

## Transaction Flow

```mermaid
---
title: Transaction Flow
---
sequenceDiagram
    participant Client
    participant Database
    participant Processor
    participant StorageManager
    participant Pager
    
    Client->>Database: execute("BEGIN")
    Database->>Processor: begin_transaction()
    Processor->>StorageManager: begin_transaction()
    StorageManager->>Pager: create_savepoint()
    Pager-->>StorageManager: savepoint_id
    
    Client->>Database: execute("INSERT ...")
    Note over Database,StorageManager: Changes buffered, not flushed
    
    Client->>Database: execute("UPDATE ...")
    Note over Database,StorageManager: More changes buffered
    
    alt COMMIT
        Client->>Database: execute("COMMIT")
        Database->>Processor: commit()
        Processor->>StorageManager: commit()
        StorageManager->>Pager: flush_dirty_pages()
        Pager->>Pager: write_all()
        Pager-->>StorageManager: Ok(())
    else ROLLBACK
        Client->>Database: execute("ROLLBACK")
        Database->>Processor: rollback()
        Processor->>StorageManager: rollback()
        StorageManager->>Pager: restore_savepoint()
        Pager->>Pager: discard_dirty_pages()
        Pager-->>StorageManager: Ok(())
    end
    
    Database-->>Client: Result<ExecutionResult>
```

## Page Cache Flow

```mermaid
---
title: Page Cache Operation
---
flowchart TD
    A[Request Page] --> B{Page in Cache?}
    B -->|Yes| C[Return Cached Page]
    B -->|No| D{Cache Full?}
    D -->|No| E[Load from Disk]
    D -->|Yes| F[Evict LRU Page]
    F --> G{Page Dirty?}
    G -->|Yes| H[Flush to Disk]
    G -->|No| I[Discard Page]
    H --> E
    I --> E
    E --> J[Add to Cache]
    J --> C
    C --> K[Update Access Time]
    K --> L[Return Page]
```

## Module Communication

```mermaid
---
title: Module Communication Pattern
---
graph LR
    subgraph "Public APIs"
        A[Database API]
        B[Query Builder]
    end
    
    subgraph "Processing"
        C[Processor]
        D[Parser]
    end
    
    subgraph "Storage"
        E[Storage Manager]
        F[Pager]
    end
    
    subgraph "OS Layer"
        G[VFS]
    end
    
    A --> C
    B --> A
    C --> D
    C --> E
    E --> F
    F --> G
    
    linkStyle 0 stroke:#2196F3,stroke-width:2px
    linkStyle 1 stroke:#4CAF50,stroke-width:2px
    linkStyle 2 stroke:#FF9800,stroke-width:2px
    linkStyle 3 stroke:#F44336,stroke-width:2px
    linkStyle 4 stroke:#9C27B0,stroke-width:2px
    linkStyle 5 stroke:#607D8B,stroke-width:2px
```

## Error Propagation

```mermaid
---
title: Error Handling Flow
---
flowchart TD
    A[Operation] --> B{Success?}
    B -->|Yes| C[Return Ok<T>]
    B -->|No| D[Create Error]
    D --> E{Error Type}
    E -->|I/O Error| F[Error::Io]
    E -->|Parse Error| G[Error::Parse]
    E -->|Type Error| H[Error::Type]
    E -->|Not Found| I[Error::NotFound]
    F --> J[Add Context]
    G --> J
    H --> J
    I --> J
    J --> K[Return Err<Error>]
    K --> L[Caller Handles]
    L --> M{Can Recover?}
    M -->|Yes| N[Continue]
    M -->|No| O[Propagate Error]
```

## Key Design Principles

1. **Unidirectional Dependencies**: Higher layers depend on lower layers, never the reverse
2. **Result Propagation**: All operations return `Result<T, Error>` for explicit error handling
3. **Lazy Loading**: Data loaded from disk only when needed
4. **Write Buffering**: Changes buffered in memory until explicit flush
5. **Cache Management**: LRU-based eviction with dirty page tracking
