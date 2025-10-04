/// Page cache - responsible for reading, writing, and caching database pages

use crate::eplite::constants::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use crate::eplite::error::{Error, Result};

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
