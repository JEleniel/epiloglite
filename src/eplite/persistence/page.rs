use epiloglite_core::PageFlags;
use flagset::FlagSet;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::calculate_crc;
use crate::eplite::persistence::{FREE_PAGE_BACK_GUARD, FREE_PAGE_FRONT_GUARD};
use crate::try_into_vec;
use crate::{CInt, eplite::SlotIndexEntry_Collection};

/// The header used by all pages in EpilogLite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageHeader {
    /// The ID of the page.
    pub page_id: CInt,
    /// Table to which this page belongs.
    pub table_id: CInt,
    /// Flags for the page.
    pub flags: FlagSet<PageFlags>,
    /// Pointer to the next overflow page, if any.
    pub next_page_id: CInt,
    /// CRC32 checksum of the page (excluding the footer).
    pub page_crc: u32,
    /// Maximum size of the page in bytes, including the header and footer.
    #[serde(skip)]
    pub max_page_size: usize,
    /// Current page size in bytes, excluding the header and footer.
    /// Used as a convenience for tracking how much space is used.
    #[serde(skip)]
    pub page_size: usize,
}

/// Represents a page in EpilogLite, containing a header and a list of entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    /// The header of the page.
    pub header: PageHeader,
    /// The lowest record_id on the page
    pub min_record_id: CInt,
    /// The highest record_id on the page
    pub max_record_id: CInt,
    /// The slot index for the page
    pub slot_index: SlotIndexEntry_Collection,
    /// The data in the page.
    pub data: Vec<u8>,
}

impl Page {
    /// Create a new page
    pub fn new(page_id: CInt, table_id: CInt, max_page_size: usize) -> Self {
        let header: PageHeader = PageHeader {
            flags: FlagSet::empty(),
            max_page_size,
            next_page_id: 0.into(),
            page_crc: 0,
            page_id,
            page_size: 0,
            table_id,
        };
        let header_size = try_into_vec(&header).unwrap().len();
        let mut new_page: Page = Page {
            header,
            data: Vec::new(),
            min_record_id: 0.into(),
            max_record_id: 0.into(),
            slot_index: SlotIndexEntry_Collection {
                slots: Vec::new(),
                fields: SlotIndexEntry_Collection::fields_metadata(),
            },
        };
        let crc = calculate_crc(&new_page);
        new_page.header.page_crc = crc;
        new_page.header.page_size = header_size; // Initial size is just the header
        new_page
    }

    /// Get a new free page filled with zeroes and guard bytes
    pub fn new_free_page(page_id: CInt, max_page_size: usize) -> Page<u32> {
        let mut zero = Page::<u32>::new(page_id, 0.into(), max_page_size);
        zero.header.flags = PageFlags::FREE.into();
        zero.entries.push(FREE_PAGE_FRONT_GUARD);
        let free_space = max_page_size - zero.header.page_size;
        zero.entries.append(&mut vec![0; free_space - 8]); // Leave space for guards
        zero.entries.push(FREE_PAGE_BACK_GUARD);
        zero
    }

    /// Is the page free?
    pub fn is_free_page(&self) -> bool {
        let bytes = try_into_vec(self.entries()).unwrap();
        self.header.flags.contains(PageFlags::FREE)
            && self.entries.len() >= 2
            && u32::from_be_bytes(bytes[0..4].try_into().unwrap()) == FREE_PAGE_FRONT_GUARD
            && u32::from_be_bytes(bytes[bytes.len() - 4..].try_into().unwrap())
                == FREE_PAGE_BACK_GUARD
    }

    /// Is the page dirty?
    pub fn is_dirty(&self) -> bool {
        self.header.flags.contains(PageFlags::DIRTY)
    }

    /// Get a reference to the entries in the page
    pub fn entries(&self) -> &Vec<T> {
        &self.entries
    }

    /// Get a clone of the entry at the specified index
    pub fn get_entry(&self, index: usize) -> Result<T, PageError> {
        if index >= self.entries.len() {
            return Err(PageError::IndexOutOfBounds(index));
        }
        Ok(self.entries[index].clone())
    }

    /// Add an entry to the page
    pub fn add_entry(&mut self, entry: T) -> Result<usize, PageError> {
        if self.header.flags.contains(PageFlags::FULL) {
            return Err(PageError::PageFull);
        }

        let entry_bytes = try_into_vec(&entry).unwrap();
        let entry_size = entry_bytes.len();

        if self.header.page_size + entry_size > self.header.max_page_size {
            self.header.flags |= PageFlags::FULL;
            return Err(PageError::PageFull);
        }

        self.entries.push(entry);
        self.header.page_size += entry_size;
        self.header.flags |= PageFlags::DIRTY;

        Ok(self.entries.len() - 1)
    }

    /// Remove an entry from the page by index
    pub fn remove_entry(&mut self, index: usize) -> Result<(), PageError> {
        if index >= self.entries.len() {
            return Err(PageError::IndexOutOfBounds(index));
        }
        let entry_bytes = try_into_vec(&self.entries[index]).unwrap();
        let entry_size = entry_bytes.len();
        self.header.page_size -= entry_size;
        self.header.flags |= PageFlags::DIRTY;

        if self.header.flags.contains(PageFlags::FULL) {
            self.header.flags ^= PageFlags::FULL;
        }
        self.entries.remove(index);

        Ok(())
    }
}

/// Errors that can occur when working with pages.
#[derive(Debug, Error)]
pub enum PageError {
    /// Index out of bounds.
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
    /// Page is full.
    #[error("Page is full")]
    PageFull,
}
