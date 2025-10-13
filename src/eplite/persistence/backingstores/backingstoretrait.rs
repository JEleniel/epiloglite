//! Trait defining the interface for a backing store, the underlying, compressed storage mechanism
use std::io;

use thiserror::Error;

/// Trait defining the interface for a backing store, the underlying, compressed storage mechanism
pub trait BackingStore {
    /// Reads a page from the backing store.
    fn read_page(&self, page_number: u128) -> Result<Vec<u8>, BackingStoreError>;
    /// Writes a page to the backing store.
    fn write_page(&mut self, page_number: u128, data: &[u8]) -> Result<(), BackingStoreError>;
    /// Allocates a new page in the backing store and returns its page number.
    fn allocate_page(&mut self) -> Result<u128, BackingStoreError>; // Returns the new page number, always allocates a free page
    /// Free a page in the backing store.
    fn free_page(&mut self, page_number: u128) -> Result<(), BackingStoreError>;

    /// Returns the total number of pages in the backing store.
    fn total_pages(&self) -> u128;
    /// Returns the number of free pages in the backing store.
    fn free_pages(&self) -> u128;
}

/// Errors that can occur when interacting with the backing store.
#[derive(Debug, Error)]
pub enum BackingStoreError {
    /// IO error reading from the backing store.
    #[error("Read IO Error: {0}")]
    ReadIOError(io::Error),
    /// IO error writing to the backing store.
    #[error("Write IO Error: {0}")]
    WriteIOError(io::Error),
    /// IO error allocating a new page in the backing store.
    #[error("Allocation IO Error: {0}")]
    AllocationIOError(io::Error),
    /// Allocation out of space in the backing store.
    #[error("Out of Space")]
    OutOfSpace,
    /// IO error freeing a page in the backing store.
    #[error("Free IO Error: {0}")]
    FreeIOError(io::Error),
    /// Attempted to read a page that does not exist.
    #[error("Page Not Found: {0}")]
    PageNotFound(u128),
    /// Attempted to access a page number that is out of bounds.
    #[error("Page Out of Bounds: {0} (total pages: {1})")]
    PageOutOfBounds(u128, u128),
    /// Attempted to overwrite a non-free page with a different page type.
    #[error("Page Type Mismatch on Write: {0}, expected {1}")]
    PageTypeMismatch(u8, u8),
}
