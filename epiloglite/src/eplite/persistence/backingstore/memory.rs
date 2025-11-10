use epiloglite_core::Cu128;
use std::collections::BTreeMap;

use crate::{
    eplite::DatabaseHeader,
    eplite::persistence::backingstore::{BackingStore, Page},
};

/// An in-memory backing store implementation used for tests and in-memory databases.
pub struct MemoryBackingStore {
    header: DatabaseHeader,
    /// In-memory map of pages keyed by canonical u128 page id
    pages: BTreeMap<u128, Page>,
    /// Next page id to allocate (reserve 0..=2 for header-like structures)
    next_page_id: u128,
    /// Recycled free pages available for allocation
    free_page_list: Vec<u128>,
}

impl MemoryBackingStore {
    /// Create a new in-memory backing store with the provided application id and migration version.
    /// The DatabaseHeader is initialized with reasonable defaults from DatabaseHeader::builder().
    pub fn new(application_id: Cu128, migration_version: Cu128) -> Self {
        // Build a default database header and set application-specific fields
        let mut database_header = DatabaseHeader::builder().expect("failed to build header");
        database_header
            .with_application_id(&application_id)
            .expect("failed to set application id");
        database_header
            .with_migration_version(&migration_version)
            .expect("failed to set migration version");

        Self {
            header: database_header,
            pages: BTreeMap::new(),
            // reserve 0..=2 for header-like structures used during boot; start allocations at 3
            next_page_id: 3u128,
            free_page_list: Vec::new(),
        }
    }
}

impl BackingStore for MemoryBackingStore {
    fn open(&mut self) -> Result<(), super::BackingStoreError> {
        // Memory backing store requires no special open actions
        Ok(())
    }

    fn flush(&mut self) -> Result<(), super::BackingStoreError> {
        // In-memory store flush is a no-op
        Ok(())
    }

    fn close(&mut self) -> Result<(), super::BackingStoreError> {
        // Nothing to close for in-memory store
        Ok(())
    }

    fn read_page(&mut self, page_id: u128) -> Result<Page, super::BackingStoreError> {
        if let Some(p) = self.pages.get(&page_id) {
            Ok(p.clone())
        } else {
            // BackingStoreError expects usize in the PageNotFound variant; cast where reasonable
            let pid_usize = page_id as usize;
            Err(super::BackingStoreError::PageNotFound(pid_usize))
        }
    }

    fn write_page(&mut self, page: Page) -> Result<(), super::BackingStoreError> {
        // Page.page_id() returns a canonical u128 page id; use it directly
        let pid: u128 = match u128::try_from(page.page_id()) {
            Ok(v) => v,
            Err(e) => {
                return Err(super::BackingStoreError::ReadIOError(
                    0,
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("CInt decode error: {:?}", e),
                    ),
                ));
            }
        };
        self.pages.insert(pid, page);
        Ok(())
    }

    fn allocate_page(&mut self) -> Result<u128, super::BackingStoreError> {
        // Try to reuse a free page first
        if let Some(recycled) = self.free_page_list.pop() {
            return Ok(recycled);
        }

        // Simple allocation strategy: return next_page_id and increment
        let allocated = self.next_page_id;
        self.next_page_id = self
            .next_page_id
            .checked_add(1)
            .ok_or(super::BackingStoreError::OutOfSpace)?;
        Ok(allocated)
    }

    fn free_page(&mut self, page_id: u128) -> Result<(), super::BackingStoreError> {
        // Remove stored page and add to recycled list
        self.pages.remove(&page_id);
        self.free_page_list.push(page_id);
        Ok(())
    }

    fn total_pages(&self) -> usize {
        // Total known pages in the backing store (allocated and stored)
        // This is an in-memory heuristic; callers should not rely on exact semantics for file-backed stores.
        self.pages.len()
    }

    fn write_journal_entry(
        &mut self,
        _entry: epiloglite_core::JournalEntry,
    ) -> Result<(), super::BackingStoreError> {
        // Journal handling not needed for the in-memory store; implement as no-op
        Ok(())
    }

    fn page_size(&self) -> usize {
        1usize << self.header.page_size_exponent()
    }
}
