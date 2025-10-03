# EpilogLite Server Mode

This document describes the server mode enhancements added in Phase 25.

## Overview

EpilogLite now includes comprehensive server-side functionality with:
- OAuth 2.0 authentication with multi-provider support
- Custom authentication handlers
- Multi-factor authentication (MFA)
- Advanced TLS 1.3 configuration
- Certificate management
- Dedicated Rust client library
- Connection pooling
- Circuit breaker pattern
- Query result caching
- Request batching

## Quick Start

### Server

```rust
use epiloglite::{Database, EpilogLiteServer, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::default();
    let server = EpilogLiteServer::new_memory(config);
    
    server.start().await?;
    Ok(())
}
```

### Client

```rust
use epiloglite::{EpilogLiteClient, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig::new("http://localhost:8080".to_string());
    let client = EpilogLiteClient::new(config);
    
    let response = client.execute("SELECT * FROM users").await?;
    println!("Success: {}", response.success);
    
    Ok(())
}
```

## Features

### Authentication

#### OAuth 2.0

```rust
use epiloglite::{OAuthConfig, OAuthProvider, AuthManager, MfaConfig};

let mut oauth_config = OAuthConfig::new("http://localhost:8080/callback".to_string());

// Register GitHub OAuth provider
oauth_config.register_provider(OAuthProvider {
    name: "github".to_string(),
    client_id: "your_client_id".to_string(),
    client_secret: "your_client_secret".to_string(),
    auth_url: "https://github.com/login/oauth/authorize".to_string(),
    token_url: "https://github.com/login/oauth/access_token".to_string(),
    profile_url: "https://api.github.com/user".to_string(),
    scopes: vec!["user:email".to_string()],
});

let mfa_config = MfaConfig::default();
let auth_manager = AuthManager::new(oauth_config, mfa_config);

// Generate OAuth authorization URL
let url = auth_manager.generate_oauth_url("github", "random_state")?;
println!("Authorize at: {}", url);
```

#### Multi-Factor Authentication (MFA)

```rust
use epiloglite::{AuthManager, OAuthConfig, MfaConfig};

let oauth_config = OAuthConfig::new("http://localhost:8080/callback".to_string());
let mfa_config = MfaConfig {
    enable_totp: true,
    enable_backup_codes: true,
    enforce_mfa: false,
    backup_code_count: 10,
};

let auth_manager = AuthManager::new(oauth_config, mfa_config);

// Generate TOTP secret for user
let totp_secret = auth_manager.generate_totp_secret("user@example.com".to_string());
println!("Scan this QR code: {}", totp_secret.provisioning_uri());

// Verify TOTP code
let is_valid = auth_manager.verify_totp(&totp_secret, "123456");

// Generate backup codes
let backup_codes = auth_manager.generate_backup_codes();
```

#### Custom Authentication Handlers

```rust
use epiloglite::{AuthHandler, UserProfile, Result};
use std::collections::HashMap;

struct MyAuthHandler;

impl AuthHandler for MyAuthHandler {
    fn authenticate(&self, credentials: &HashMap<String, String>) -> Result<UserProfile> {
        // Your authentication logic here
        Ok(UserProfile {
            id: "user123".to_string(),
            email: Some("user@example.com".to_string()),
            name: Some("User Name".to_string()),
            avatar_url: None,
            provider: "custom".to_string(),
        })
    }

    fn validate_token(&self, token: &str) -> Result<UserProfile> {
        // Your token validation logic here
        Ok(UserProfile {
            id: "user123".to_string(),
            email: Some("user@example.com".to_string()),
            name: Some("User Name".to_string()),
            avatar_url: None,
            provider: "custom".to_string(),
        })
    }

    fn name(&self) -> &str {
        "my_custom_auth"
    }
}

// Register the handler
let mut auth_manager = AuthManager::new(oauth_config, mfa_config);
auth_manager.register_handler(Box::new(MyAuthHandler));
```

### TLS Configuration

```rust
use epiloglite::{TlsConfig, TlsVersion, ClientCertMode, CipherSuite};

let tls_config = TlsConfig::new()
    .with_cert_and_key(
        "/path/to/cert.pem".to_string(),
        "/path/to/key.pem".to_string()
    )
    .with_version(TlsVersion::Tls13)
    .with_client_cert_mode(ClientCertMode::Required)
    .with_cipher_suites(CipherSuite::Strong)
    .with_rotation(3600); // Check for rotation every hour

tls_config.validate()?;
```

### Certificate Management

```rust
use epiloglite::{CertificateManager, Certificate, TlsConfig};

let tls_config = TlsConfig::default();
let mut cert_manager = CertificateManager::new(tls_config)?;

// Upload a certificate
cert_manager.upload_certificate(Certificate {
    id: "server-cert".to_string(),
    common_name: "example.com".to_string(),
    data: "-----BEGIN CERTIFICATE-----\n...".to_string(),
    key: Some("-----BEGIN PRIVATE KEY-----\n...".to_string()),
    expires_at: "2025-12-31T23:59:59Z".to_string(),
    chain: vec![],
})?;

// List all certificates
let certs = cert_manager.list_certificates();

// Revoke a certificate
cert_manager.revoke_certificate("server-cert")?;

// Check if renewal is needed
let needs_renewal = cert_manager.needs_renewal("server-cert")?;
```

### Client Library

#### Connection Pooling

```rust
use epiloglite::{ClientConfig, EpilogLiteClient};

let config = ClientConfig::new("http://localhost:8080".to_string())
    .with_pool_size(20)
    .with_retry_config(5, 100, 10000);

let client = EpilogLiteClient::new(config);

// Get pool statistics
let stats = client.pool_stats();
println!("Total connections: {}", stats.total_connections);
println!("Healthy connections: {}", stats.healthy_connections);
```

#### Request Builder

```rust
use epiloglite::{RequestBuilder, EpilogLiteClient};

let request = RequestBuilder::new()
    .sql("SELECT * FROM users WHERE age > ?".to_string())
    .param("age".to_string(), "18".to_string())
    .build()?;

let response = client.execute_request(request).await?;
```

#### Circuit Breaker

```rust
use epiloglite::{EpilogLiteClient, CircuitState};

let client = EpilogLiteClient::new(config);

// Check circuit breaker state
match client.circuit_state() {
    CircuitState::Closed => println!("Circuit is healthy"),
    CircuitState::Open => println!("Circuit is open, requests will fail fast"),
    CircuitState::HalfOpen => println!("Circuit is testing recovery"),
}
```

### Performance Optimization

#### Query Caching

```rust
use epiloglite::{QueryCache, CacheConfig, CacheEvictionPolicy};

let cache_config = CacheConfig {
    enabled: true,
    max_size: 1000,
    ttl_seconds: 300,
    eviction_policy: CacheEvictionPolicy::Lru,
    enable_stats: true,
};

let cache = QueryCache::new(cache_config);

// Put value in cache
cache.put("query1".to_string(), serde_json::json!({"result": "data"}))?;

// Get value from cache
if let Some(value) = cache.get("query1") {
    println!("Cache hit: {:?}", value);
}

// Invalidate specific key
cache.invalidate("query1")?;

// Invalidate by pattern
cache.invalidate_pattern("user:")?;

// Get cache statistics
let stats = cache.stats();
println!("Hit rate: {:.2}%", stats.hit_rate);
```

#### Request Batching

```rust
use epiloglite::{BatchProcessor, BatchConfig, BatchRequest};

let batch_config = BatchConfig {
    enabled: true,
    max_batch_size: 100,
    batch_timeout_ms: 1000,
    enable_parallel: true,
};

let processor = BatchProcessor::new(batch_config);

let request = BatchRequest {
    statements: vec![
        "INSERT INTO users (name) VALUES ('Alice')".to_string(),
        "INSERT INTO users (name) VALUES ('Bob')".to_string(),
        "UPDATE users SET active = true".to_string(),
    ],
    use_transaction: true,
};

let response = processor.process(request).await?;
println!("Executed {} statements in {}ms", 
    response.results.len(), 
    response.total_time_ms
);
```

## API Reference

### REST API Endpoints

- `GET /health` - Health check endpoint
- `POST /api/execute` - Execute SQL statement
- `POST /api/auth` - Authenticate user

### Request/Response Format

#### Execute SQL

Request:
```json
{
  "sql": "SELECT * FROM users"
}
```

Response:
```json
{
  "success": true,
  "message": "Query executed successfully",
  "rows_affected": 1
}
```

## Configuration

### Environment Variables

- `EPILOGLITE_BIND_ADDR` - Server bind address (default: "127.0.0.1:8080")
- `EPILOGLITE_TLS_CERT` - Path to TLS certificate
- `EPILOGLITE_TLS_KEY` - Path to TLS private key
- `EPILOGLITE_JWT_SECRET` - Secret for JWT token signing
- `EPILOGLITE_DB_PATH` - Path to database file

### Server Configuration

```rust
use epiloglite::ServerConfig;

let config = ServerConfig {
    bind_addr: "0.0.0.0:8080".to_string(),
    tls_cert_path: Some("/path/to/cert.pem".to_string()),
    tls_key_path: Some("/path/to/key.pem".to_string()),
    enable_rest: true,
    enable_graphql: false, // Temporarily disabled
    jwt_secret: "your-secret-key".to_string(),
};
```

## Security Considerations

### Authentication

- Always use HTTPS in production
- Store OAuth client secrets securely
- Rotate JWT secrets regularly
- Enforce MFA for sensitive operations

### TLS

- Use TLS 1.3 for best security
- Keep certificates up to date
- Monitor certificate expiration
- Use strong cipher suites

### Database

- Use parameterized queries to prevent SQL injection
- Implement proper access control
- Audit database operations
- Encrypt sensitive data at rest

## Testing

Run tests with server feature enabled:

```bash
cargo test --features server
```

Run specific server tests:

```bash
cargo test --features server server::
```

## Examples

See the `examples/` directory for complete examples:

- `examples/server_basic.rs` - Basic server setup
- `examples/server_oauth.rs` - OAuth authentication
- `examples/server_mfa.rs` - Multi-factor authentication
- `examples/client_basic.rs` - Basic client usage
- `examples/client_pooling.rs` - Connection pooling
- `examples/caching.rs` - Query caching
- `examples/batching.rs` - Request batching

## Limitations

- Database operations are not thread-safe by default
- GraphQL support temporarily disabled due to dependency conflicts
- Client certificate validation is simplified
- TOTP implementation is basic (production use requires proper library)

## Future Enhancements

- WebSocket support for real-time updates
- Advanced rate limiting
- Distributed caching
- Prometheus metrics
- OpenTelemetry tracing
- GraphQL support (once dependency conflicts resolved)

## License

Same as EpilogLite main project (LGPL-3.0-only)
