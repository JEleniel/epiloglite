//! In-memory backing store implementation.

use crate::persistence::{
    FREE_PAGE_BACK_GUARD, FREE_PAGE_FRONT_GUARD, FreePage,
    backingstores::{BackingStore, BackingStoreError},
};

/// In-memory backing store
/// Because vectors cannot be indexed with u128 directly, we split pages into low and high parts.
pub struct MemoryBackingStore {
    page_size: usize,
    low_pages: Vec<Vec<u8>>,  // Each page is a vector of bytes
    high_pages: Vec<Vec<u8>>, // Each page is a vector of bytes
}

impl MemoryBackingStore {
    /// Create a new in-memory backing store
    pub fn new() -> Self {
        MemoryBackingStore {
            page_size: 4096,
            low_pages: Vec::new(),
            high_pages: Vec::new(),
        }
    }

    fn split_page_number(page_number: u128) -> (usize, bool) {
        if page_number > (u64::MAX as u128) {
            let index = ((page_number >> 64) - 1) as usize;
            (index, true)
        } else {
            let index = page_number as usize;
            (index, false)
        }
    }

    fn is_free_page(&self, page_number: u128) -> Result<bool, BackingStoreError> {
        if page_number >= self.total_pages() {
            return Err(super::BackingStoreError::PageOutOfBounds(
                page_number,
                self.total_pages(),
            ));
        }

        let (index, is_high) = Self::split_page_number(page_number);

        if is_high {
            Ok(
                self.high_pages[index][0..4] == FREE_PAGE_FRONT_GUARD.to_be_bytes()
                    && self.high_pages[index][self.page_size - 4..]
                        == FREE_PAGE_BACK_GUARD.to_be_bytes(),
            )
        } else {
            Ok(
                self.low_pages[index][0..4] == FREE_PAGE_FRONT_GUARD.to_be_bytes()
                    && self.low_pages[index][self.page_size - 4..]
                        == FREE_PAGE_BACK_GUARD.to_be_bytes(),
            )
        }
    }
}

impl BackingStore for MemoryBackingStore {
    fn read_page(&self, page_number: u128) -> Result<Vec<u8>, super::BackingStoreError> {
        if page_number >= self.total_pages() {
            return Err(super::BackingStoreError::PageOutOfBounds(
                page_number,
                self.total_pages(),
            ));
        }
        let (index, is_high) = Self::split_page_number(page_number);
        if is_high {
            Ok(self.high_pages[index].clone())
        } else {
            Ok(self.low_pages[index].clone())
        }
    }

    fn write_page(&mut self, page_number: u128, data: &[u8]) -> Result<(), BackingStoreError> {
        if page_number >= self.total_pages() {
            return Err(super::BackingStoreError::PageOutOfBounds(
                page_number,
                self.total_pages(),
            ));
        }
        let (index, is_high) = Self::split_page_number(page_number);
        if is_high {
            if self.high_pages[index][0..4] != FREE_PAGE_FRONT_GUARD.to_be_bytes()
                && self.high_pages[index][0] != data[0]
            {
                return Err(BackingStoreError::PageTypeMismatch(
                    data[0],
                    self.high_pages[index][0],
                ));
            }
            self.high_pages[index] = data.to_vec();
        } else {
            if self.low_pages[index][0..4] != FREE_PAGE_FRONT_GUARD.to_be_bytes()
                && self.low_pages[index][0] != data[0]
            {
                return Err(BackingStoreError::PageTypeMismatch(
                    data[0],
                    self.low_pages[index][0],
                ));
            }
            self.low_pages[index] = data.to_vec();
        }
        Ok(())
    }

    fn allocate_page(&mut self) -> Result<u128, BackingStoreError> {
        let mut page_number: u128 = 0;

        // Skip pages 0 and 1 (reserved for header and accounting)
        for i in 2..self.total_pages() {
            if let Ok(_) = self.is_free_page(page_number) {
                page_number = i;
                break;
            }
        }
        if page_number == 0 || page_number == u128::MAX - 1 {
            // Either the free page at the end is missing, or we ran out of space
            return Err(BackingStoreError::OutOfSpace);
        }

        if page_number == self.total_pages() - 1 {
            if page_number < (u64::MAX as u128) {
                self.low_pages
                    .push(FreePage::new(self.page_size).to_bytes());
            } else {
                self.high_pages
                    .push(FreePage::new(self.page_size).to_bytes());
            }
        } else {
            return Err(BackingStoreError::OutOfSpace);
        }

        Ok(page_number)
    }

    fn free_page(&mut self, page_number: u128) -> Result<(), super::BackingStoreError> {
        if page_number >= self.total_pages() {
            return Err(super::BackingStoreError::PageOutOfBounds(
                page_number,
                self.total_pages(),
            ));
        }
        let (index, is_high) = Self::split_page_number(page_number);
        if is_high {
            self.high_pages[index] = FreePage::new(self.page_size).to_bytes();
        } else {
            self.low_pages[index] = FreePage::new(self.page_size).to_bytes();
        }
        Ok(())
    }

    fn total_pages(&self) -> u128 {
        (self.low_pages.len() + self.high_pages.len()) as u128
    }

    fn free_pages(&self) -> u128 {
        todo!()
    }
}
