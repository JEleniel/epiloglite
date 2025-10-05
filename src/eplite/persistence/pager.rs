/// Page cache - responsible for reading, writing, and caching database pages

use crate::eplite::constants::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use crate::eplite::error::{Error, Result};
use crate::eplite::persistence::wal::{WalWriter, WalReader, WalFrame, WalCheckpoint, CheckpointMode};

#[cfg(feature = "std")]
use crate::eplite::traits::file::File;
#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(not(feature = "std"))]
use alloc::{
	collections::BTreeMap as HashMap,
	format,
	string::{String, ToString},
	vec,
	vec::Vec,
};

/// Journal mode for database
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalMode {
	/// Rollback journal mode (traditional)
	Rollback,
	/// Write-Ahead Log mode (concurrent access)
	Wal,
}

/// A single page in the database
#[derive(Debug, Clone)]
pub struct Page {
	pub page_number: u32,
	pub data: Vec<u8>,
	pub dirty: bool,
}

impl Page {
	pub fn new(page_number: u32, size: usize) -> Self {
		Page {
			page_number,
			data: vec![0; size],
			dirty: false,
		}
	}

	/// Mark the page as modified
	pub fn mark_dirty(&mut self) {
		self.dirty = true;
	}

	/// Write data to the page
	pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<()> {
		if offset + data.len() > self.data.len() {
			return Err(Error::Internal(format!(
				"Write beyond page boundary: offset={}, len={}, page_size={}",
				offset,
				data.len(),
				self.data.len()
			)));
		}
		self.data[offset..offset + data.len()].copy_from_slice(data);
		self.mark_dirty();
		Ok(())
	}

	/// Read data from the page
	pub fn read(&self, offset: usize, len: usize) -> Result<&[u8]> {
		if offset + len > self.data.len() {
			return Err(Error::Internal(format!(
				"Read beyond page boundary: offset={}, len={}, page_size={}",
				offset, len, self.data.len()
			)));
		}
		Ok(&self.data[offset..offset + len])
	}
}

/// Page cache manager
#[derive(Debug)]
pub struct Pager {
	page_size: u32,
	cache: HashMap<u32, Page>,
	max_cache_size: usize,
	#[cfg(feature = "std")]
	file: Option<Box<dyn File>>,
	journal_mode: JournalMode,
	#[cfg(feature = "std")]
	wal_file: Option<Box<dyn File>>,
	wal_writer: Option<WalWriter>,
	in_transaction: bool,
}

impl Pager {
	pub fn new(page_size: u32) -> Result<Self> {
		if !(MIN_PAGE_SIZE..=MAX_PAGE_SIZE).contains(&page_size) {
			return Err(Error::InvalidFormat(format!(
				"Invalid page size: {}. Must be between {} and {}",
				page_size, MIN_PAGE_SIZE, MAX_PAGE_SIZE
			)));
		}

		if !page_size.is_power_of_two() {
			return Err(Error::InvalidFormat(format!(
				"Page size must be a power of two: {}",
				page_size
			)));
		}

		Ok(Pager {
			page_size,
			cache: HashMap::new(),
			max_cache_size: 100,
			#[cfg(feature = "std")]
			file: None,
			journal_mode: JournalMode::Rollback,
			#[cfg(feature = "std")]
			wal_file: None,
			wal_writer: None,
			in_transaction: false,
		})
	}

	/// Create a pager with a file backend
	#[cfg(feature = "std")]
	pub fn with_file(page_size: u32, file: Box<dyn File>) -> Result<Self> {
		let mut pager = Self::new(page_size)?;
		pager.file = Some(file);
		Ok(pager)
	}

	pub fn page_size(&self) -> u32 {
		self.page_size
	}

	/// Get a page from cache or load it
	pub fn get_page(&mut self, page_number: u32) -> Result<&Page> {
		if !self.cache.contains_key(&page_number) {
			let page = self.load_page(page_number)?;
			
			// Evict pages if cache is too large
			if self.cache.len() >= self.max_cache_size {
				self.evict_page()?;
			}
			
			self.cache.insert(page_number, page);
		}
		Ok(self.cache.get(&page_number).unwrap())
	}

	/// Get a mutable page from cache or load it
	pub fn get_page_mut(&mut self, page_number: u32) -> Result<&mut Page> {
		if !self.cache.contains_key(&page_number) {
			let page = self.load_page(page_number)?;
			
			// Evict pages if cache is too large
			if self.cache.len() >= self.max_cache_size {
				self.evict_page()?;
			}
			
			self.cache.insert(page_number, page);
		}
		Ok(self.cache.get_mut(&page_number).unwrap())
	}

	fn load_page(&mut self, page_number: u32) -> Result<Page> {
		#[cfg(feature = "std")]
		{
			if self.journal_mode == JournalMode::Wal {
				return self.load_page_with_wal(page_number);
			}
		}

		let mut page = Page::new(page_number, self.page_size as usize);
		
		#[cfg(feature = "std")]
		if let Some(file) = &mut self.file {
			// Calculate offset in file
			let offset = (page_number as u64) * (self.page_size as u64);
			
			// Try to read the page from disk
			match file.read(offset) {
				Ok(data) if !data.is_empty() => {
					let len = data.len().min(page.data.len());
					page.data[..len].copy_from_slice(&data[..len]);
				}
				Ok(_) => {
					// Empty file or beyond EOF, page stays zeroed
				}
				Err(e) => {
					// If read fails, treat as empty page
					eprintln!("Warning: Failed to read page {}: {}", page_number, e);
				}
			}
		}
		
		Ok(page)
	}

	/// Evict a clean page from cache (LRU-like policy)
	fn evict_page(&mut self) -> Result<()> {
		// Find first non-dirty page to evict
		if let Some(page_num) = self.cache.iter()
			.find(|(_, p)| !p.dirty)
			.map(|(k, _)| *k) {
			self.cache.remove(&page_num);
		} else {
			// If all pages are dirty, flush one
			if let Some((&page_num, _)) = self.cache.iter().next() {
				// Write the page first
				if let Some(page) = self.cache.get(&page_num) {
					#[cfg(feature = "std")]
					if let Some(file) = &mut self.file {
						let offset = (page.page_number as u64) * (self.page_size as u64);
						file.write(&page.data, offset)?;
					}
				}
				// Then mark it clean
				if let Some(page) = self.cache.get_mut(&page_num) {
					page.dirty = false;
				}
			}
		}
		Ok(())
	}

	/// Flush dirty pages to disk
	pub fn flush(&mut self) -> Result<()> {
		#[cfg(feature = "std")]
		{
			if self.journal_mode == JournalMode::Wal && self.in_transaction {
				return self.flush_wal();
			}
		}

		// Collect dirty pages to write
		let dirty_pages: Vec<(u32, Vec<u8>)> = self.cache.iter()
			.filter(|(_, p)| p.dirty)
			.map(|(num, p)| (*num, p.data.clone()))
			.collect();
		
		// Write them
		for (page_num, data) in dirty_pages {
			#[cfg(feature = "std")]
			if let Some(file) = &mut self.file {
				let offset = (page_num as u64) * (self.page_size as u64);
				file.write(&data, offset)?;
			}
			// Mark as clean
			if let Some(page) = self.cache.get_mut(&page_num) {
				page.dirty = false;
			}
		}
		
		// Sync file to disk
		#[cfg(feature = "std")]
		if let Some(file) = &mut self.file {
			use crate::eplite::traits::file::SynchronizationType;
			use flagset::FlagSet;
			file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))?;
		}
		
		Ok(())
	}

	/// Clear the cache
	pub fn clear_cache(&mut self) -> Result<()> {
		self.flush()?;
		self.cache.clear();
		Ok(())
	}

	/// Allocate a new page
	pub fn allocate_page(&mut self) -> Result<u32> {
		// Find the first unused page number
		let mut page_num = 1u32;
		while self.cache.contains_key(&page_num) {
			page_num += 1;
			if page_num == 0 {
				return Err(Error::Internal("Page number overflow".to_string()));
			}
		}
		
		// Create the new page
		let page = Page::new(page_num, self.page_size as usize);
		self.cache.insert(page_num, page);
		
		Ok(page_num)
	}

	/// Set journal mode
	#[cfg(feature = "std")]
	pub fn set_journal_mode(&mut self, mode: JournalMode, wal_file: Option<Box<dyn File>>) -> Result<()> {
		// Flush any pending changes before switching modes
		self.flush()?;
		
		self.journal_mode = mode;
		
		match mode {
			JournalMode::Wal => {
				self.wal_file = wal_file;
				self.wal_writer = Some(WalWriter::new(self.page_size));
			}
			JournalMode::Rollback => {
				self.wal_file = None;
				self.wal_writer = None;
			}
		}
		
		Ok(())
	}

	/// Get current journal mode
	pub fn journal_mode(&self) -> JournalMode {
		self.journal_mode
	}

	/// Begin a transaction (WAL mode)
	pub fn begin_transaction(&mut self) -> Result<()> {
		if self.in_transaction {
			return Err(Error::Internal("Already in transaction".to_string()));
		}
		self.in_transaction = true;
		Ok(())
	}

	/// Commit a transaction (WAL mode)
	pub fn commit_transaction(&mut self) -> Result<()> {
		if !self.in_transaction {
			return Err(Error::Internal("Not in transaction".to_string()));
		}

		if self.journal_mode == JournalMode::Wal {
			self.flush_wal()?;
		} else {
			self.flush()?;
		}

		self.in_transaction = false;
		Ok(())
	}

	/// Rollback a transaction
	pub fn rollback_transaction(&mut self) -> Result<()> {
		if !self.in_transaction {
			return Err(Error::Internal("Not in transaction".to_string()));
		}

		// Collect page numbers that need reloading
		let dirty_pages: Vec<u32> = self.cache.iter()
			.filter(|(_, p)| p.dirty)
			.map(|(num, _)| *num)
			.collect();

		// Reload pages from disk or WAL
		for page_num in dirty_pages {
			let reloaded_page = self.load_page(page_num)?;
			if let Some(page) = self.cache.get_mut(&page_num) {
				*page = reloaded_page;
			}
		}

		self.in_transaction = false;
		Ok(())
	}

	/// Flush dirty pages to WAL
	#[cfg(feature = "std")]
	fn flush_wal(&mut self) -> Result<()> {
		if let Some(wal_writer) = &mut self.wal_writer {
			// Collect dirty pages
			let dirty_pages: Vec<(u32, Vec<u8>)> = self.cache.iter()
				.filter(|(_, p)| p.dirty)
				.map(|(num, p)| (*num, p.data.clone()))
				.collect();

			if dirty_pages.is_empty() {
				return Ok(());
			}

			// Add frames to WAL
			for (page_num, data) in dirty_pages {
				let frame = WalFrame::new(page_num, data, 0, 0);
				wal_writer.add_frame(frame)?;
			}

			// Commit the transaction
			let db_size = self.cache.len() as u32;
			wal_writer.commit(db_size)?;

			// Write WAL to file
			if let Some(wal_file) = &mut self.wal_file {
				let wal_bytes = wal_writer.to_bytes();
				wal_file.write(&wal_bytes, 0)?;
				
				use crate::eplite::traits::file::SynchronizationType;
				use flagset::FlagSet;
				wal_file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))?;
			}

			// Mark pages as clean
			for page in self.cache.values_mut() {
				page.dirty = false;
			}
		}

		Ok(())
	}

	/// Perform a checkpoint (transfer WAL to database)
	#[cfg(feature = "std")]
	pub fn checkpoint(&mut self, mode: CheckpointMode) -> Result<()> {
		if self.journal_mode != JournalMode::Wal {
			return Ok(());
		}

		// Read the WAL file
		if let Some(wal_file) = &mut self.wal_file {
			let wal_bytes = wal_file.read(0)?;
			
			if wal_bytes.is_empty() {
				return Ok(());
			}

			let reader = WalReader::from_bytes(&wal_bytes)?;
			let checkpoint = WalCheckpoint::new(self.page_size);
			let (updates, _result) = checkpoint.checkpoint(&reader, mode)?;

			// Apply updates to database file
			if let Some(file) = &mut self.file {
				for (page_num, data) in updates {
					let offset = (page_num as u64) * (self.page_size as u64);
					file.write(&data, offset)?;
				}
				
				use crate::eplite::traits::file::SynchronizationType;
				use flagset::FlagSet;
				file.sync(FlagSet::from(SynchronizationType::SqliteSyncFull))?;
			}

			// Reset WAL if requested
			if mode == CheckpointMode::Restart || mode == CheckpointMode::Truncate {
				if let Some(wal_writer) = &mut self.wal_writer {
					wal_writer.reset();
				}
				// Truncate WAL file
				wal_file.truncate(0)?;
			}
		}

		Ok(())
	}

	/// Load page from database or WAL
	#[cfg(feature = "std")]
	fn load_page_with_wal(&mut self, page_number: u32) -> Result<Page> {
		let mut page = Page::new(page_number, self.page_size as usize);

		// Try to read from WAL first if in WAL mode
		if self.journal_mode == JournalMode::Wal {
			if let Some(wal_file) = &mut self.wal_file {
				let wal_bytes = wal_file.read(0)?;
				if !wal_bytes.is_empty() {
					if let Ok(reader) = WalReader::from_bytes(&wal_bytes) {
						if let Some(data) = reader.get_page(page_number) {
							page.data[..data.len()].copy_from_slice(data);
							return Ok(page);
						}
					}
				}
			}
		}

		// Fall back to reading from main database file
		if let Some(file) = &mut self.file {
			let offset = (page_number as u64) * (self.page_size as u64);
			match file.read(offset) {
				Ok(data) if !data.is_empty() => {
					let len = data.len().min(page.data.len());
					page.data[..len].copy_from_slice(&data[..len]);
				}
				Ok(_) => {
					// Empty or beyond EOF
				}
				Err(_) => {
					// Read failed, return empty page
				}
			}
		}

		Ok(page)
	}
}

impl Default for Pager {
	fn default() -> Self {
		Self::new(DEFAULT_PAGE_SIZE).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_pager_creation() {
		let pager = Pager::new(4096);
		assert!(pager.is_ok());
		let pager = pager.unwrap();
		assert_eq!(pager.page_size(), 4096);
	}

	#[test]
	fn test_invalid_page_size() {
		let pager = Pager::new(1000);
		assert!(pager.is_err());
	}

	#[test]
	fn test_page_creation() {
		let page = Page::new(1, 4096);
		assert_eq!(page.page_number, 1);
		assert_eq!(page.data.len(), 4096);
		assert!(!page.dirty);
	}

	#[test]
	fn test_get_page() {
		let mut pager = Pager::new(4096).unwrap();
		let page = pager.get_page(1);
		assert!(page.is_ok());
		assert_eq!(page.unwrap().page_number, 1);
	}

	#[test]
	fn test_page_write_read() {
		let mut page = Page::new(1, 4096);
		let data = b"Hello, World!";
		page.write(0, data).unwrap();
		assert!(page.dirty);
		
		let read_data = page.read(0, data.len()).unwrap();
		assert_eq!(read_data, data);
	}

	#[test]
	fn test_page_write_beyond_boundary() {
		let mut page = Page::new(1, 100);
		let data = vec![0u8; 200];
		let result = page.write(0, &data);
		assert!(result.is_err());
	}

	#[test]
	fn test_allocate_page() {
		let mut pager = Pager::new(4096).unwrap();
		let page1 = pager.allocate_page().unwrap();
		let page2 = pager.allocate_page().unwrap();
		assert_ne!(page1, page2);
	}
}
