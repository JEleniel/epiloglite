# EpilogLite Deployment Architecture

## Overview

This document describes the various deployment scenarios for EpilogLite and their architectural implications.

## Embedded Library Mode

```mermaid
---
title: Embedded Library Deployment
---
graph TB
    subgraph "Application Process"
        A[Application Code]
        B[EpilogLite Library]
        C[Database File]
    end
    
    subgraph "Operating System"
        D[File System]
        E[VFS Layer]
    end
    
    A --> B
    B --> C
    C --> D
    B --> E
    E --> D
    
    style A fill:#e1f5ff
    style B fill:#fff4e1
    style C fill:#e1ffe1
    style D fill:#ffe1e1
```

**Characteristics:**
- EpilogLite compiled into application
- Direct file access via VFS
- Single process access
- Minimal overhead
- Suitable for: desktop apps, CLI tools, mobile apps

## Standalone Server Mode

```mermaid
---
title: Standalone Server Deployment
---
graph TB
    subgraph "Client Applications"
        A1[Web App]
        A2[Mobile App]
        A3[Desktop App]
    end
    
    subgraph "Network Layer"
        B[Load Balancer]
        C[TLS Termination]
    end
    
    subgraph "EpilogLite Server"
        D[REST/GraphQL API]
        E[Authentication]
        F[Authorization]
        G[EpilogLite Core]
        H[Database Files]
    end
    
    subgraph "Infrastructure"
        I[File System]
        J[Monitoring]
        K[Logging]
    end
    
    A1 --> B
    A2 --> B
    A3 --> B
    B --> C
    C --> D
    D --> E
    E --> F
    F --> G
    G --> H
    H --> I
    G --> J
    G --> K
    
    style D fill:#f0e1ff
    style E fill:#ffe1e1
    style F fill:#ffe1e1
    style G fill:#fff4e1
```

**Characteristics:**
- EpilogLite as standalone service
- Network API (REST/GraphQL)
- Multi-client access
- Authentication and authorization
- Suitable for: web applications, microservices, multi-user systems

## C API Integration Mode

```mermaid
---
title: C API Integration Deployment
---
graph TB
    subgraph "C/C++ Application"
        A[Application Code]
        B[SQLite API Calls]
    end
    
    subgraph "EpilogLite C ABI"
        C[C API Shim]
        D[EpilogLite Core]
    end
    
    subgraph "Storage"
        E[Database File]
    end
    
    A --> B
    B --> C
    C --> D
    D --> E
    
    style A fill:#e1f5ff
    style B fill:#e1e1ff
    style C fill:#ffe1f0
    style D fill:#fff4e1
```

**Characteristics:**
- Drop-in replacement for SQLite
- C ABI compatibility
- Existing C/C++ code unchanged
- Feature-gated compilation
- Suitable for: legacy applications, C/C++ projects

## Microservice Architecture

```mermaid
---
title: Microservice Integration
---
graph LR
    subgraph "API Gateway"
        A[Gateway]
    end
    
    subgraph "Microservices"
        B[Auth Service]
        C[User Service]
        D[Order Service]
    end
    
    subgraph "Data Layer"
        E[EpilogLite 1]
        F[EpilogLite 2]
        G[EpilogLite 3]
    end
    
    A --> B
    A --> C
    A --> D
    
    B --> E
    C --> F
    D --> G
    
    style B fill:#e1f5ff
    style C fill:#e1f5ff
    style D fill:#e1f5ff
    style E fill:#fff4e1
    style F fill:#fff4e1
    style G fill:#fff4e1
```

**Characteristics:**
- Each microservice has own database
- Database per service pattern
- Embedded library mode per service
- Service isolation
- Suitable for: microservices, distributed systems

## Multi-Tier Architecture

```mermaid
---
title: Multi-Tier Deployment
---
graph TB
    subgraph "Presentation Tier"
        A[Web Browser]
        B[Mobile App]
    end
    
    subgraph "Application Tier"
        C[Web Server]
        D[App Server]
    end
    
    subgraph "Business Logic Tier"
        E[Business Logic]
        F[ORM Layer]
    end
    
    subgraph "Data Tier"
        G[EpilogLite]
        H[Database Files]
    end
    
    A --> C
    B --> D
    C --> E
    D --> E
    E --> F
    F --> G
    G --> H
    
    style E fill:#e1f5ff
    style F fill:#e1ffe1
    style G fill:#fff4e1
```

**Characteristics:**
- Clear separation of concerns
- ORM layer for object mapping
- Business logic isolated
- Database abstracted
- Suitable for: enterprise applications, web applications

## High Availability Setup (Planned)

```mermaid
---
title: High Availability Architecture (Planned)
---
graph TB
    subgraph "Clients"
        A[Client Apps]
    end
    
    subgraph "Load Balancing"
        B[Load Balancer]
    end
    
    subgraph "Application Cluster"
        C[EpilogLite Server 1]
        D[EpilogLite Server 2]
        E[EpilogLite Server 3]
    end
    
    subgraph "Storage Layer"
        F[Shared Storage / Replication]
        G[Primary Database]
        H[Replica 1]
        I[Replica 2]
    end
    
    A --> B
    B --> C
    B --> D
    B --> E
    
    C --> G
    D --> G
    E --> G
    
    G -.->|Replication| H
    G -.->|Replication| I
    
    style C fill:#fff4e1
    style D fill:#fff4e1
    style E fill:#fff4e1
    style G fill:#e1ffe1
    style H fill:#ffe1e1
    style I fill:#ffe1e1
```

**Characteristics:**
- Multiple server instances
- Load balancing
- Database replication
- Failover capability
- Status: Planned
- Suitable for: mission-critical applications

## Platform Support Matrix

| Platform | Embedded Mode | Server Mode | C API Mode | Status |
|----------|--------------|-------------|------------|--------|
| Linux x86_64 | ✅ | ✅ | ✅ | Production |
| Linux ARM64 | ✅ | ✅ | ✅ | Production |
| Windows x64 | ✅ | ✅ | ✅ | Production |
| macOS x64 | ✅ | ✅ | ✅ | Production |
| macOS ARM64 | ✅ | ✅ | ✅ | Production |
| BSD | ✅ | 🚧 | ✅ | Beta |
| Android | ✅ | ❌ | ✅ | Planned |
| iOS | ✅ | ❌ | ✅ | Planned |
| WASM | 🚧 | ❌ | ❌ | Planned |
| Embedded (no-std) | 🚧 | ❌ | ❌ | Planned |

Legend:
- ✅ Fully Supported
- 🚧 In Progress
- ❌ Not Supported

## Deployment Considerations

### Embedded Mode
- **Pros**: Minimal overhead, simple deployment, no network latency
- **Cons**: Single process access, no multi-user support
- **Best For**: Desktop applications, mobile apps, CLI tools

### Server Mode
- **Pros**: Multi-user support, centralized data, remote access
- **Cons**: Network overhead, more complex deployment, requires authentication
- **Best For**: Web applications, multi-client scenarios

### C API Mode
- **Pros**: Drop-in SQLite replacement, existing code compatibility
- **Cons**: C ABI overhead, limited to C features
- **Best For**: Legacy applications, C/C++ projects

### Microservices
- **Pros**: Service isolation, independent scaling, fault isolation
- **Cons**: Multiple databases, eventual consistency challenges
- **Best For**: Distributed systems, cloud-native applications
