/// Async file implementation using tokio

#[cfg(feature = "async")]
use crate::eplite::traits::async_file::AsyncFile;
#[cfg(feature = "async")]
use crate::eplite::traits::file::{LockType, SynchronizationType, UnlockType};
#[cfg(feature = "async")]
use async_trait::async_trait;
#[cfg(feature = "async")]
use flagset::FlagSet;
#[cfg(feature = "async")]
use std::io;
#[cfg(feature = "async")]
use std::path::Path;
#[cfg(feature = "async")]
use tokio::fs::File as TokioFile;
#[cfg(feature = "async")]
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

/// Async file implementation using tokio
#[cfg(feature = "async")]
#[derive(Debug)]
pub struct AsyncDefaultFile {
	file: TokioFile,
	path: String,
}

#[cfg(feature = "async")]
impl AsyncDefaultFile {
	/// Open a file asynchronously
	pub async fn open<P: AsRef<Path>>(
		path: P,
		read: bool,
		write: bool,
		create: bool,
	) -> io::Result<Self> {
		let path_str = path.as_ref().to_string_lossy().to_string();
		let file = tokio::fs::OpenOptions::new()
			.read(read)
			.write(write)
			.create(create)
			.open(path)
			.await?;

		Ok(AsyncDefaultFile {
			file,
			path: path_str,
		})
	}

	/// Get the file path
	pub fn path(&self) -> &str {
		&self.path
	}
}

#[cfg(feature = "async")]
#[async_trait]
impl AsyncFile for AsyncDefaultFile {
	async fn close(&mut self) -> io::Result<()> {
		// File is automatically closed when dropped
		Ok(())
	}

	async fn read(&mut self, offset: u64) -> io::Result<Vec<u8>> {
		self.file.seek(io::SeekFrom::Start(offset)).await?;
		let mut buffer = Vec::new();
		self.file.read_to_end(&mut buffer).await?;
		Ok(buffer)
	}

	async fn write(&mut self, data: &[u8], offset: u64) -> io::Result<()> {
		self.file.seek(io::SeekFrom::Start(offset)).await?;
		self.file.write_all(data).await?;
		Ok(())
	}

	async fn truncate(&mut self, size: u64) -> io::Result<()> {
		self.file.set_len(size).await?;
		Ok(())
	}

	async fn sync(&mut self, flags: FlagSet<SynchronizationType>) -> io::Result<()> {
		if flags.contains(SynchronizationType::SqliteSyncFull) {
			self.file.sync_all().await?;
		} else {
			// Both SqliteSyncDataonly and default case use sync_data
			self.file.sync_data().await?;
		}
		Ok(())
	}

	async fn file_size(&mut self) -> io::Result<u64> {
		let metadata = self.file.metadata().await?;
		Ok(metadata.len())
	}

	async fn lock(&mut self, _lock_type: LockType) -> io::Result<()> {
		// TODO: Implement platform-specific async locking
		// For now, this is a placeholder that doesn't block
		Ok(())
	}

	async fn unlock(&mut self, _unlock_type: UnlockType) -> io::Result<()> {
		// TODO: Implement platform-specific async locking
		// For now, this is a placeholder that doesn't block
		Ok(())
	}

	async fn check_reserved_lock(&mut self) -> io::Result<u64> {
		// TODO: Implement platform-specific async lock checking
		// For now, this is a placeholder that doesn't block
		Ok(0)
	}
}

#[cfg(all(test, feature = "async"))]
mod tests {
	use super::*;

	#[tokio::test]
	async fn test_async_file_open_read_write() {
		let temp_path = "/tmp/epiloglite_async_test_file.db";

		// Clean up any existing file
		let _ = tokio::fs::remove_file(temp_path).await;

		// Create and write to file
		{
			let mut file = AsyncDefaultFile::open(temp_path, true, true, true)
				.await
				.unwrap();
			let data = b"Hello, Async World!";
			file.write(data, 0).await.unwrap();
			file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))
				.await
				.unwrap();
		}

		// Read from file
		{
			let mut file = AsyncDefaultFile::open(temp_path, true, false, false)
				.await
				.unwrap();
			let data = file.read(0).await.unwrap();
			assert_eq!(&data[..], b"Hello, Async World!");
		}

		// Clean up
		tokio::fs::remove_file(temp_path).await.unwrap();
	}

	#[tokio::test]
	async fn test_async_file_size() {
		let temp_path = "/tmp/epiloglite_async_test_size.db";
		let _ = tokio::fs::remove_file(temp_path).await;

		{
			let mut file = AsyncDefaultFile::open(temp_path, true, true, true)
				.await
				.unwrap();
			let data = b"Test data";
			file.write(data, 0).await.unwrap();
			let size = file.file_size().await.unwrap();
			assert_eq!(size, 9);
		}

		tokio::fs::remove_file(temp_path).await.unwrap();
	}

	#[tokio::test]
	async fn test_async_file_truncate() {
		let temp_path = "/tmp/epiloglite_async_test_truncate.db";
		let _ = tokio::fs::remove_file(temp_path).await;

		{
			let mut file = AsyncDefaultFile::open(temp_path, true, true, true)
				.await
				.unwrap();
			let data = b"Test data with more content";
			file.write(data, 0).await.unwrap();
			file.truncate(9).await.unwrap();
			let size = file.file_size().await.unwrap();
			assert_eq!(size, 9);
		}

		tokio::fs::remove_file(temp_path).await.unwrap();
	}

	#[tokio::test]
	async fn test_async_file_concurrent_operations() {
		let temp_path1 = "/tmp/epiloglite_async_test_concurrent1.db";
		let temp_path2 = "/tmp/epiloglite_async_test_concurrent2.db";

		// Clean up
		let _ = tokio::fs::remove_file(temp_path1).await;
		let _ = tokio::fs::remove_file(temp_path2).await;

		// Spawn two concurrent file operations
		let task1 = tokio::spawn(async move {
			let mut file = AsyncDefaultFile::open(temp_path1, true, true, true)
				.await
				.unwrap();
			let data = b"File 1 data";
			file.write(data, 0).await.unwrap();
			file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))
				.await
				.unwrap();
		});

		let task2 = tokio::spawn(async move {
			let mut file = AsyncDefaultFile::open(temp_path2, true, true, true)
				.await
				.unwrap();
			let data = b"File 2 data";
			file.write(data, 0).await.unwrap();
			file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))
				.await
				.unwrap();
		});

		// Wait for both tasks to complete
		let _ = tokio::try_join!(task1, task2);

		// Clean up
		let _ = tokio::fs::remove_file(temp_path1).await;
		let _ = tokio::fs::remove_file(temp_path2).await;
	}

	#[tokio::test]
	async fn test_async_file_path() {
		let temp_path = "/tmp/epiloglite_async_test_path.db";
		let _ = tokio::fs::remove_file(temp_path).await;

		let file = AsyncDefaultFile::open(temp_path, true, true, true)
			.await
			.unwrap();
		assert_eq!(file.path(), temp_path);

		tokio::fs::remove_file(temp_path).await.unwrap();
	}
}
