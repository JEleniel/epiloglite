#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod eplite;

#[cfg(feature = "capi")]
pub mod capi;

pub use eplite::command::processor::ExecutionResult;
pub use eplite::database::Database;
pub use eplite::error::{Error, Result};
pub use eplite::query_builder::{
	CreateTableBuilder, DeleteBuilder, InsertBuilder, SelectBuilder, UpdateBuilder,
};
pub use eplite::{SchemaFormat, TextEncoding};

#[cfg(feature = "async")]
pub use eplite::os::{async_file, async_vfs, backpressure, performance};

#[cfg(feature = "async")]
pub use eplite::traits::async_file::AsyncFile;

pub use eplite::traits::file::{File, LockType, SynchronizationType, UnlockType};
pub use eplite::os::vfs::{AccessFlags, OpenFlags, VirtualFileSystem};

#[cfg(feature = "server")]
pub use eplite::server::{
	AuthHandler, AuthManager, BackupCode, BatchConfig, BatchProcessor, BatchRequest,
	BatchResponse, CacheConfig, CacheEvictionPolicy, Certificate, CertificateManager,
	CertValidationPolicy, CipherSuite, CircuitState, ClientCertMode, ClientConfig,
	ConnectionState, EpilogLiteClient, EpilogLiteServer, MfaConfig, OAuthConfig,
	OAuthProvider, PoolStats, QueryCache, RequestBuilder, ServerConfig, ServerState,
	SqlRequest, SqlResponse, TlsConfig, TlsVersion, TotpSecret, UserProfile,
};

pub const EPILOGLITE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SQLITE_SHM_NLOCK: u32 = 0;

pub enum SerializeFlags {
    SqliteSerializeNocopy = 0x01,
}
