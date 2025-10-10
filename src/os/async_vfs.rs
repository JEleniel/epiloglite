/// Async Virtual File System trait

use crate::eplite::error::Result;
use crate::eplite::os::vfs::{AccessFlags, OpenFlags};

#[cfg(feature = "async")]
use async_trait::async_trait;

#[cfg(feature = "async")]
use crate::eplite::traits::async_file::AsyncFile;

#[cfg(feature = "std")]
use std::path::Path;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// Async Virtual File System trait for non-blocking operations
#[cfg(feature = "async")]
#[async_trait]
pub trait AsyncVirtualFileSystem: Send + Sync {
	/// Open or create a file asynchronously
	async fn open(&self, path: &Path, flags: OpenFlags) -> Result<Box<dyn AsyncFile>>;

	/// Delete a file asynchronously
	async fn delete(&self, path: &Path, sync_dir: bool) -> Result<()>;

	/// Check if a file exists and is accessible asynchronously
	async fn access(&self, path: &Path, flags: AccessFlags) -> Result<bool>;

	/// Get the full pathname of a file asynchronously
	async fn full_pathname(&self, path: &Path) -> Result<String>;

	/// Sleep for a given number of microseconds asynchronously
	async fn sleep(&self, microseconds: u64) -> Result<u64>;

	/// Get current time asynchronously
	async fn current_time(&self) -> Result<i64>;

	/// Generate random bytes asynchronously
	async fn randomness(&self, buffer: &mut [u8]) -> Result<usize>;
}

/// Default async VFS implementation using tokio
#[cfg(feature = "async")]
pub struct AsyncDefaultVfs {}

#[cfg(feature = "async")]
impl AsyncDefaultVfs {
	pub fn new() -> Self {
		AsyncDefaultVfs {}
	}
}

#[cfg(feature = "async")]
impl Default for AsyncDefaultVfs {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(feature = "async")]
#[async_trait]
impl AsyncVirtualFileSystem for AsyncDefaultVfs {
	async fn open(&self, path: &Path, flags: OpenFlags) -> Result<Box<dyn AsyncFile>> {
		use super::async_file::AsyncDefaultFile;
		
		let file = AsyncDefaultFile::open(
			path,
			flags.read_only || flags.read_write,
			flags.read_write || flags.create,
			flags.create,
		)
		.await?;
		Ok(Box::new(file))
	}

	async fn delete(&self, path: &Path, _sync_dir: bool) -> Result<()> {
		tokio::fs::remove_file(path).await?;
		Ok(())
	}

	async fn access(&self, path: &Path, _flags: AccessFlags) -> Result<bool> {
		Ok(tokio::fs::try_exists(path).await?)
	}

	async fn full_pathname(&self, path: &Path) -> Result<String> {
		let canonical = tokio::fs::canonicalize(path).await?;
		Ok(canonical.to_string_lossy().to_string())
	}

	async fn sleep(&self, microseconds: u64) -> Result<u64> {
		tokio::time::sleep(tokio::time::Duration::from_micros(microseconds)).await;
		Ok(microseconds)
	}

	async fn current_time(&self) -> Result<i64> {
		let now = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap();
		Ok(now.as_secs() as i64)
	}

	async fn randomness(&self, buffer: &mut [u8]) -> Result<usize> {
		// Use tokio's spawn_blocking for CPU-bound random generation
		let len = buffer.len();
		let random_bytes = tokio::task::spawn_blocking(move || {
			use rand::RngCore;
			let mut rng = rand::thread_rng();
			let mut bytes = vec![0u8; len];
			rng.fill_bytes(&mut bytes);
			bytes
		})
		.await
		.map_err(|e| crate::eplite::error::Error::Internal(format!("Random generation error: {}", e)))?;
		
		buffer.copy_from_slice(&random_bytes);
		Ok(buffer.len())
	}
}

#[cfg(all(test, feature = "async"))]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_async_default_vfs() {
		let vfs = AsyncDefaultVfs::new();
		let time = vfs.current_time().await;
		assert!(time.is_ok());
	}

	#[tokio::test]
	async fn test_async_sleep() {
		let vfs = AsyncDefaultVfs::new();
		let start = std::time::Instant::now();
		let _ = vfs.sleep(10000).await; // 10ms
		let elapsed = start.elapsed();
		assert!(elapsed.as_micros() >= 10000);
	}

	#[tokio::test]
	async fn test_async_randomness() {
		let vfs = AsyncDefaultVfs::new();
		let mut buffer = [0u8; 16];
		let result = vfs.randomness(&mut buffer).await;
		assert!(result.is_ok());
		assert_eq!(result.unwrap(), 16);
		// Check that not all bytes are zero (very unlikely with real random data)
		assert!(buffer.iter().any(|&b| b != 0));
	}

	#[tokio::test]
	async fn test_async_access() {
		let vfs = AsyncDefaultVfs::new();
		let path = Path::new("/tmp");
		let result = vfs.access(path, AccessFlags::exists()).await;
		assert!(result.is_ok());
	}
}
