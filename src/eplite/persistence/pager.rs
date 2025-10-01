/// Page cache - responsible for reading, writing, and caching database pages

use crate::eplite::constants::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE, MIN_PAGE_SIZE};
use crate::eplite::error::{Error, Result};
use std::collections::HashMap;

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
}

/// Page cache manager
pub struct Pager {
	page_size: u32,
	cache: HashMap<u32, Page>,
	max_cache_size: usize,
}

impl Pager {
	pub fn new(page_size: u32) -> Result<Self> {
		if page_size < MIN_PAGE_SIZE || page_size > MAX_PAGE_SIZE {
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
		})
	}

	pub fn page_size(&self) -> u32 {
		self.page_size
	}

	/// Get a page from cache or load it
	pub fn get_page(&mut self, page_number: u32) -> Result<&Page> {
		if !self.cache.contains_key(&page_number) {
			let page = self.load_page(page_number)?;
			self.cache.insert(page_number, page);
		}
		Ok(self.cache.get(&page_number).unwrap())
	}

	/// Get a mutable page from cache or load it
	pub fn get_page_mut(&mut self, page_number: u32) -> Result<&mut Page> {
		if !self.cache.contains_key(&page_number) {
			let page = self.load_page(page_number)?;
			self.cache.insert(page_number, page);
		}
		Ok(self.cache.get_mut(&page_number).unwrap())
	}

	fn load_page(&self, page_number: u32) -> Result<Page> {
		// TODO: Load from disk
		Ok(Page::new(page_number, self.page_size as usize))
	}

	/// Flush dirty pages to disk
	pub fn flush(&mut self) -> Result<()> {
		// TODO: Write dirty pages to disk
		for page in self.cache.values_mut() {
			if page.dirty {
				// Write to disk
				page.dirty = false;
			}
		}
		Ok(())
	}

	/// Clear the cache
	pub fn clear_cache(&mut self) -> Result<()> {
		self.flush()?;
		self.cache.clear();
		Ok(())
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
}
