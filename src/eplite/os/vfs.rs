/// Virtual File System trait

use crate::eplite::error::Result;
use crate::eplite::traits::file::{File, LockType, SynchronizationType, UnlockType};
use flagset::FlagSet;
use std::io;
use std::path::Path;

/// Virtual File System trait
pub trait VirtualFileSystem {
	/// Open or create a file
	fn open(&self, path: &Path, flags: OpenFlags) -> Result<Box<dyn File>>;

	/// Delete a file
	fn delete(&self, path: &Path, sync_dir: bool) -> Result<()>;

	/// Check if a file exists and is accessible
	fn access(&self, path: &Path, flags: AccessFlags) -> Result<bool>;

	/// Get the full pathname of a file
	fn full_pathname(&self, path: &Path) -> Result<String>;

	/// Sleep for a given number of microseconds
	fn sleep(&self, microseconds: u64) -> Result<u64>;

	/// Get current time
	fn current_time(&self) -> Result<i64>;

	/// Generate random bytes
	fn randomness(&self, buffer: &mut [u8]) -> Result<usize>;
}

/// File open flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenFlags {
	pub read_only: bool,
	pub read_write: bool,
	pub create: bool,
	pub exclusive: bool,
	pub truncate: bool,
}

impl OpenFlags {
	pub fn read_only() -> Self {
		OpenFlags {
			read_only: true,
			read_write: false,
			create: false,
			exclusive: false,
			truncate: false,
		}
	}

	pub fn read_write() -> Self {
		OpenFlags {
			read_only: false,
			read_write: true,
			create: false,
			exclusive: false,
			truncate: false,
		}
	}

	pub fn create() -> Self {
		OpenFlags {
			read_only: false,
			read_write: true,
			create: true,
			exclusive: false,
			truncate: false,
		}
	}
}

/// File access check flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessFlags {
	pub exists: bool,
	pub read: bool,
	pub write: bool,
}

impl AccessFlags {
	pub fn exists() -> Self {
		AccessFlags {
			exists: true,
			read: false,
			write: false,
		}
	}

	pub fn read() -> Self {
		AccessFlags {
			exists: true,
			read: true,
			write: false,
		}
	}

	pub fn write() -> Self {
		AccessFlags {
			exists: true,
			read: false,
			write: true,
		}
	}
}

/// Default VFS implementation using standard library
pub struct DefaultVfs {}

impl DefaultVfs {
	pub fn new() -> Self {
		DefaultVfs {}
	}
}

impl Default for DefaultVfs {
	fn default() -> Self {
		Self::new()
	}
}

impl VirtualFileSystem for DefaultVfs {
	fn open(&self, _path: &Path, _flags: OpenFlags) -> Result<Box<dyn File>> {
		// TODO: Implement file opening
		Err(crate::eplite::error::Error::NotSupported(
			"File opening not yet implemented".to_string(),
		))
	}

	fn delete(&self, path: &Path, _sync_dir: bool) -> Result<()> {
		std::fs::remove_file(path)?;
		Ok(())
	}

	fn access(&self, path: &Path, _flags: AccessFlags) -> Result<bool> {
		Ok(path.exists())
	}

	fn full_pathname(&self, path: &Path) -> Result<String> {
		let canonical = std::fs::canonicalize(path)?;
		Ok(canonical.to_string_lossy().to_string())
	}

	fn sleep(&self, microseconds: u64) -> Result<u64> {
		std::thread::sleep(std::time::Duration::from_micros(microseconds));
		Ok(microseconds)
	}

	fn current_time(&self) -> Result<i64> {
		let now = std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap();
		Ok(now.as_secs() as i64)
	}

	fn randomness(&self, buffer: &mut [u8]) -> Result<usize> {
		// TODO: Use a cryptographically secure random number generator
		for byte in buffer.iter_mut() {
			*byte = 0;
		}
		Ok(buffer.len())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_open_flags() {
		let flags = OpenFlags::read_only();
		assert!(flags.read_only);
		assert!(!flags.read_write);
	}

	#[test]
	fn test_access_flags() {
		let flags = AccessFlags::exists();
		assert!(flags.exists);
		assert!(!flags.read);
	}

	#[test]
	fn test_default_vfs() {
		let vfs = DefaultVfs::new();
		let time = vfs.current_time();
		assert!(time.is_ok());
	}
}
