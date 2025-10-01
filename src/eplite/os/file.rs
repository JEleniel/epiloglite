/// Default file implementation using standard library

use crate::eplite::traits::file::{File, LockType, SynchronizationType, UnlockType};
use flagset::FlagSet;
use std::fs::{File as StdFile, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// Default file implementation
pub struct DefaultFile {
	file: StdFile,
	path: String,
}

impl DefaultFile {
	/// Open a file
	pub fn open<P: AsRef<Path>>(path: P, read: bool, write: bool, create: bool) -> io::Result<Self> {
		let path_str = path.as_ref().to_string_lossy().to_string();
		let file = OpenOptions::new()
			.read(read)
			.write(write)
			.create(create)
			.open(path)?;
		
		Ok(DefaultFile {
			file,
			path: path_str,
		})
	}

	/// Get the file path
	pub fn path(&self) -> &str {
		&self.path
	}
}

impl File for DefaultFile {
	fn close(&mut self) -> io::Result<()> {
		// File is automatically closed when dropped
		Ok(())
	}

	fn read(&mut self, offset: u64) -> io::Result<Vec<u8>> {
		self.file.seek(SeekFrom::Start(offset))?;
		let mut buffer = Vec::new();
		self.file.read_to_end(&mut buffer)?;
		Ok(buffer)
	}

	fn write(&mut self, data: &Vec<u8>, offset: u64) -> io::Result<()> {
		self.file.seek(SeekFrom::Start(offset))?;
		self.file.write_all(data)?;
		Ok(())
	}

	fn truncate(&mut self, size: u64) -> io::Result<()> {
		self.file.set_len(size)?;
		Ok(())
	}

	fn sync(&mut self, flags: FlagSet<SynchronizationType>) -> io::Result<()> {
		if flags.contains(SynchronizationType::SqliteSyncFull) {
			self.file.sync_all()?;
		} else if flags.contains(SynchronizationType::SqliteSyncDataonly) {
			self.file.sync_data()?;
		} else {
			self.file.sync_data()?;
		}
		Ok(())
	}

	fn file_size(&mut self) -> io::Result<u64> {
		Ok(self.file.metadata()?.len())
	}

	fn lock(&mut self, _lock_type: LockType) -> io::Result<()> {
		// TODO: Implement platform-specific locking
		Ok(())
	}

	fn unlock(&mut self, _unlock_type: UnlockType) -> io::Result<()> {
		// TODO: Implement platform-specific locking
		Ok(())
	}

	fn check_reserved_lock(&mut self) -> io::Result<u64> {
		// TODO: Implement platform-specific lock checking
		Ok(0)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::Write as _;

	#[test]
	fn test_file_open_read_write() {
		let temp_path = "/tmp/epiloglite_test_file.db";
		
		// Clean up any existing file
		let _ = std::fs::remove_file(temp_path);
		
		// Create and write to file
		{
			let mut file = DefaultFile::open(temp_path, true, true, true).unwrap();
			let data = b"Hello, World!".to_vec();
			file.write(&data, 0).unwrap();
			file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull)).unwrap();
		}
		
		// Read from file
		{
			let mut file = DefaultFile::open(temp_path, true, false, false).unwrap();
			let data = file.read(0).unwrap();
			assert_eq!(&data[..], b"Hello, World!");
		}
		
		// Clean up
		std::fs::remove_file(temp_path).unwrap();
	}

	#[test]
	fn test_file_size() {
		let temp_path = "/tmp/epiloglite_test_size.db";
		let _ = std::fs::remove_file(temp_path);
		
		{
			let mut file = DefaultFile::open(temp_path, true, true, true).unwrap();
			let data = b"Test data".to_vec();
			file.write(&data, 0).unwrap();
			let size = file.file_size().unwrap();
			assert_eq!(size, 9);
		}
		
		std::fs::remove_file(temp_path).unwrap();
	}

	#[test]
	fn test_file_truncate() {
		let temp_path = "/tmp/epiloglite_test_truncate.db";
		let _ = std::fs::remove_file(temp_path);
		
		{
			let mut file = DefaultFile::open(temp_path, true, true, true).unwrap();
			let data = b"Test data with more content".to_vec();
			file.write(&data, 0).unwrap();
			file.truncate(9).unwrap();
			let size = file.file_size().unwrap();
			assert_eq!(size, 9);
		}
		
		std::fs::remove_file(temp_path).unwrap();
	}
}
