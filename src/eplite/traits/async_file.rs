/// Async file trait for non-blocking I/O operations

use flagset::FlagSet;

#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "std")]
use std::io;
#[cfg(feature = "std")]
use std::fmt;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(not(feature = "std"))]
use core::fmt;

use super::file::{LockType, SynchronizationType, UnlockType};

/// Type alias for I/O errors
#[cfg(feature = "std")]
pub type IoError = io::Error;

#[cfg(not(feature = "std"))]
pub type IoError = i32;

/// Async file trait for non-blocking file operations
#[cfg(feature = "async")]
#[async_trait]
pub trait AsyncFile: fmt::Debug + Send + Sync {
	/// Close the file asynchronously
	async fn close(&mut self) -> Result<(), IoError>;

	/// Read from the file at a specific offset asynchronously
	async fn read(&mut self, offset: u64) -> Result<Vec<u8>, IoError>;

	/// Write to the file at a specific offset asynchronously
	async fn write(&mut self, data: &[u8], offset: u64) -> Result<(), IoError>;

	/// Truncate the file to a specific size asynchronously
	async fn truncate(&mut self, size: u64) -> Result<(), IoError>;

	/// Sync file to disk asynchronously
	async fn sync(&mut self, flags: FlagSet<SynchronizationType>) -> Result<(), IoError>;

	/// Get file size asynchronously
	async fn file_size(&mut self) -> Result<u64, IoError>;

	/// Lock the file asynchronously
	async fn lock(&mut self, lock_type: LockType) -> Result<(), IoError>;

	/// Unlock the file asynchronously
	async fn unlock(&mut self, unlock_type: UnlockType) -> Result<(), IoError>;

	/// Check if the file has a reserved lock asynchronously
	async fn check_reserved_lock(&mut self) -> Result<u64, IoError>;
}
