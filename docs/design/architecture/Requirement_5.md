# Server Mode and Network APIs

## Overview

EpilogLite must support standalone server mode with REST and GraphQL APIs, enabling network-based database access with authentication, authorization, and TLS encryption for multi-client scenarios.

## User Story

As a system architect, I need to deploy EpilogLite as a standalone database server with secure network access so that multiple applications can share a central database with proper access control and encryption.

## Features

### 1. REST API
- HTTP endpoints for SQL execution
- JSON request/response format
- CRUD operations via HTTP methods
- Transaction management over HTTP
- Feature-gated (server feature)

**Acceptance Criteria:**
- POST /execute endpoint accepts SQL statements
- GET endpoints for read-only queries
- HTTP status codes reflect operation results
- Request/response format is well-documented
- Error responses include helpful messages

### 2. GraphQL API
- GraphQL schema for database operations
- Queries for data retrieval
- Mutations for data modification
- Subscriptions for real-time updates (planned)
- GraphiQL playground for development
- Feature-gated (server feature)

**Acceptance Criteria:**
- GraphQL schema covers all database operations
- Queries return properly structured data
- Mutations handle transactions correctly
- GraphiQL interface works for testing
- Schema introspection available

### 3. Authentication and Authorization
- JWT-based authentication
- Role-based access control (RBAC)
- Table-level permissions
- Operation-level permissions (SELECT, INSERT, UPDATE, DELETE)
- User and role management

**Acceptance Criteria:**
- JWT tokens issued on successful login
- Invalid tokens rejected
- Permissions checked before operations
- RBAC model supports complex scenarios
- Admin operations require elevated privileges

### 4. TLS/SSL Encryption
- TLS 1.3 support via rustls
- Certificate management
- Client certificate authentication (optional)
- Secure by default configuration
- Performance optimized

**Acceptance Criteria:**
- All network traffic encrypted by default
- TLS 1.3 negotiation works correctly
- Certificate validation enforced
- Performance overhead acceptable (<10%)
- Secure cipher suites only

### 5. Client Libraries
- Official Rust client library
- HTTP client for any language
- Connection pooling
- Automatic reconnection
- Request batching

**Acceptance Criteria:**
- Rust client provides idiomatic API
- Connection pool manages resources efficiently
- Reconnection handles network interruptions
- Batch requests reduce round-trips
- Documentation includes examples for multiple languages
