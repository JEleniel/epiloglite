//! Server module for EpilogLite
//!
//! This module provides server-side functionality including:
//! - REST API and GraphQL endpoints
//! - Authentication (OAuth, custom handlers, MFA)
//! - TLS configuration
//! - Client library
//! - Performance optimizations (caching, batching)

#[cfg(feature = "server")]
pub mod api;

#[cfg(feature = "server")]
pub mod auth;

#[cfg(feature = "server")]
pub mod tls;

#[cfg(feature = "server")]
pub mod client;

#[cfg(feature = "server")]
pub mod performance;

// Re-export commonly used types
#[cfg(feature = "server")]
pub use api::{EpilogLiteServer, ServerConfig, ServerState};

#[cfg(feature = "server")]
pub use auth::{
	AuthHandler, AuthManager, BackupCode, MfaConfig, OAuthConfig, OAuthProvider, TotpSecret,
	UserProfile,
};

#[cfg(feature = "server")]
pub use tls::{
	Certificate, CertificateManager, CertValidationPolicy, CipherSuite, ClientCertMode,
	TlsConfig, TlsVersion,
};

#[cfg(feature = "server")]
pub use client::{
	ClientConfig, CircuitState, ConnectionState, EpilogLiteClient, PoolStats, RequestBuilder,
	SqlRequest, SqlResponse,
};

#[cfg(feature = "server")]
pub use performance::{
	BatchConfig, BatchProcessor, BatchRequest, BatchResponse, CacheConfig, CacheEvictionPolicy,
	QueryCache,
};
