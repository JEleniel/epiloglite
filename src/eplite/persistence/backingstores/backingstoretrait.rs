//! Trait defining the interface for a backing store, the underlying, compressed storage mechanism
use std::{fmt::Debug, io, path::PathBuf};

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

use crate::CInt;
use crate::eplite::JournalEntry;
use crate::eplite::persistence::Page;

/// Trait defining the interface for a backing store, the underlying, compressed storage mechanism
pub trait BackingStore {
    /// Prepares the backing store for use, performing any necessary initialization.
    fn open(&mut self, create: bool) -> Result<(), BackingStoreError>;

    /// Flushes any buffered data to the backing store, ensuring all writes are persisted.
    fn flush(&mut self) -> Result<(), BackingStoreError>;

    /// Closes the backing store, releasing any resources and ensuring all data is saved.
    fn close(&mut self) -> Result<(), BackingStoreError>;

    /// Reads a page from the backing store.
    fn read_page<T>(&mut self, page_id: usize) -> Result<Page<T>, BackingStoreError>
    where
        T: Serialize + DeserializeOwned + Debug + Clone;

    /// Writes a page to the backing store.
    fn write_page<T>(&mut self, page: Page<T>) -> Result<(), BackingStoreError>
    where
        T: Serialize + DeserializeOwned + Debug + Clone;

    /// Allocates a new page in the backing store and returns its data
    fn allocate_page<T>(&mut self) -> Result<T, BackingStoreError>
    where
        T: Serialize + DeserializeOwned + Debug + Clone;

    /// Free a page in the backing store.
    fn free_page(&mut self, page_id: usize) -> Result<(), BackingStoreError>;

    /// Returns the total number of pages in the backing store.
    fn total_pages(&self) -> usize;

    /// Returns the number of free pages in the backing store.
    fn free_pages(&self) -> usize;

    /// Writes a Journal entry to the backing store
    /// This is separate from other writes because it must be executed immediately
    /// and cannot be buffered or delayed.
    fn write_journal_entry(&mut self, entry: JournalEntry) -> Result<(), BackingStoreError>;
}

/// Errors that can occur when interacting with the backing store.
#[derive(Debug, Error)]
pub enum BackingStoreError {
    /// Error decoding data from the backing store.
    #[error("An error occurred while decoding data page {0}: {1:?}")]
    DecodeError(usize, bincode::error::DecodeError),
    /// Error encoding data to the backing store.
    #[error("An error occurred while encoding data page {0}: {1:?}")]
    EncodeError(usize, bincode::error::EncodeError),
    /// IO error reading from the backing store.
    #[error("An error occurred while reading page {0}: {1:?}")]
    ReadIOError(usize, io::Error),
    /// General IO error.
    #[error("An IO error occurred: {0}")]
    IoError(io::Error),
    /// IO error writing to the backing store.
    #[error("An error occurred while writing page {0}: {1:?}")]
    WriteIOError(usize, io::Error),
    /// IO error allocating a new page in the backing store.
    #[error("An error occurred while allocating a new page: {0}")]
    AllocationIOError(io::Error),
    /// Allocation out of space in the backing store.
    #[error("The backing store is out of space")]
    OutOfSpace,
    /// IO error freeing a page in the backing store.
    #[error("An error occurred while freeing page {0}: {1:?}")]
    FreeIOError(usize, io::Error),
    /// Attempted to read a page that does not exist.
    #[error("Page {0} does not exist")]
    PageNotFound(usize),
    /// Attempted to access a page number that is out of bounds.
    #[error("Page {0} is out of bounds: total pages {1}")]
    PageOutOfBounds(usize, usize),
    /// Attempted to overwrite a non-free page with a different table's data
    #[error("Table {0} owns the page, table {1} attempted to overwrite it")]
    TableIdMismatch(usize, usize),
    /// Not a data page.
    #[error("Page {0} is not a data page")]
    NotADataPage(usize),
    /// Header fails validation (checksum or fields)
    #[error("The database header checksum failed: expected {0:08X}, found {1:08X}")]
    InvalidHeader(u32, u32),
    /// The primary and secondary headers do not match
    #[error("Database Headers do not match: primary CRC {0:08X}, secondary CRC {1:08X}")]
    HeaderMismatch(u32, u32),
    /// The application ID does not match the expected value
    #[error("Application ID does not match the actual value, expected {0}, found {1}")]
    ApplicationIdMismatch(CInt, CInt),
    /// The migration version does not match the expected value
    #[error("Migration Version does not match: expected <={0}, found {1}")]
    MigrationVersionMismatch(CInt, CInt),
    /// Failed to write a Journal entry
    #[error("An error occurred while writing a journal entry: {0}")]
    JournalWriteError(String),
    /// File not found
    #[error("The backing store file {0} was not found")]
    FileNotFound(PathBuf),
    /// Cannot create file because it already exists
    #[error("The backing store file {0} already exists")]
    FileExists(PathBuf),
}

impl From<io::Error> for BackingStoreError {
    fn from(err: io::Error) -> Self {
        BackingStoreError::ReadIOError(0, err)
    }
}

impl From<bincode::error::DecodeError> for BackingStoreError {
    fn from(err: bincode::error::DecodeError) -> Self {
        BackingStoreError::DecodeError(0, err)
    }
}

impl From<bincode::error::EncodeError> for BackingStoreError {
    fn from(err: bincode::error::EncodeError) -> Self {
        BackingStoreError::EncodeError(0, err)
    }
}
