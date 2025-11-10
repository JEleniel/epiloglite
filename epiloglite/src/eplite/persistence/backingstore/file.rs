use std::collections::BTreeMap;
use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use crate::eplite::DatabaseHeader;
use crate::eplite::persistence::backingstore::PageCache;
use crate::eplite::persistence::backingstore::{BackingStore, Page};
use epiloglite_core::{Cu128, try_from_slice, try_into_vec};
use std::sync::{Arc, Mutex};

/// Simple file-backed backing store.
///
/// This implementation serializes individual `Page` values with a 8-byte
/// length prefix (big-endian u64) followed by the bincode-encoded bytes of the
/// Page. A lightweight in-memory index maps page ids to file offsets; when a
/// page is overwritten with a different size the new bytes are appended and the
/// index updated (leaving old data as garbage). This keeps implementation
/// straightforward while allowing reads and writes by page id.
pub struct FileBackingStore {
    file_path: PathBuf,
    page_cache_size: usize,
    create: bool,
    file: Option<File>,
    header: DatabaseHeader,
    /// Map of page_id -> file offset
    page_index: BTreeMap<u128, u64>,
    /// Next page id to allocate
    next_page_id: u128,
}

impl FileBackingStore {
    /// Create a new FileBackingStore (not opened). If `create` is true the
    /// parent directories will be created when opening the store.
    pub fn new<P: Into<PathBuf>>(
        path: P,
        application_id: Cu128,
        migration_version: Cu128,
        page_cache_size: usize,
        create: bool,
    ) -> Result<Self, super::BackingStoreError> {
        let mut header = DatabaseHeader::builder().map_err(|e| {
            super::BackingStoreError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("failed to build header: {:?}", e),
            ))
        })?;
        header.with_application_id(&application_id).map_err(|e| {
            super::BackingStoreError::IoError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            ))
        })?;
        header
            .with_migration_version(&migration_version)
            .map_err(|e| {
                super::BackingStoreError::IoError(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{:?}", e),
                ))
            })?;

        Ok(Self {
            file_path: path.into(),
            page_cache_size,
            create,
            file: None,
            header,
            page_index: BTreeMap::new(),
            next_page_id: 3u128,
        })
    }

    /// Convenience constructor: create a FileBackingStore and wrap it in a
    /// PageCache for improved performance. Returns the configured PageCache.
    pub fn with_cache<P: Into<PathBuf>>(
        path: P,
        application_id: Cu128,
        migration_version: Cu128,
        cache_pages: usize,
        create: bool,
    ) -> Result<PageCache<FileBackingStore>, super::BackingStoreError> {
        let bs =
            FileBackingStore::new(path, application_id, migration_version, cache_pages, create)?;
        let arc = Arc::new(Mutex::new(bs));
        Ok(PageCache::new(arc, cache_pages))
    }

    fn open_file(&mut self) -> Result<(), super::BackingStoreError> {
        if self.file.is_some() {
            return Ok(());
        }

        if let Some(parent) = self.file_path.parent() {
            if self.create {
                create_dir_all(parent).map_err(|e| super::BackingStoreError::IoError(e))?;
            }
        }

        let mut f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(self.create)
            .open(&self.file_path)
            .map_err(|e| super::BackingStoreError::IoError(e))?;

        // Build index by scanning file sequentially
        let mut offset: u64 = 0;
        loop {
            let mut len_buf = [0u8; 8];
            match f.read_exact(&mut len_buf) {
                Ok(_) => {
                    let len = u64::from_be_bytes(len_buf) as usize;
                    let mut buf = vec![0u8; len];
                    f.read_exact(&mut buf)
                        .map_err(|e| super::BackingStoreError::ReadIOError(0, e))?;
                    match try_from_slice::<Page>(&buf) {
                        Ok(page) => {
                            self.page_index.insert(page.page_id(), offset);
                            offset = offset
                                .checked_add(8)
                                .and_then(|o| o.checked_add(len as u64))
                                .ok_or(super::BackingStoreError::IoError(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    "file offset overflow",
                                )))?;
                        }
                        Err(_) => break,
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(super::BackingStoreError::ReadIOError(0, e)),
            }
        }

        // set next_page_id heuristically as max seen + 1
        if let Some(max) = self.page_index.keys().max().cloned() {
            self.next_page_id = max.saturating_add(1);
        }

        // Re-open file handle for future seeks/writes
        let f2 = OpenOptions::new()
            .read(true)
            .write(true)
            .create(self.create)
            .open(&self.file_path)
            .map_err(|e| super::BackingStoreError::IoError(e))?;
        self.file = Some(f2);
        Ok(())
    }
}

impl BackingStore for FileBackingStore {
    fn open(&mut self) -> Result<(), super::BackingStoreError> {
        self.open_file()
    }

    fn flush(&mut self) -> Result<(), super::BackingStoreError> {
        if let Some(f) = &mut self.file {
            f.flush()
                .map_err(|e| super::BackingStoreError::WriteIOError(0, e))?;
        }
        Ok(())
    }

    fn close(&mut self) -> Result<(), super::BackingStoreError> {
        if let Some(mut f) = self.file.take() {
            f.flush()
                .map_err(|e| super::BackingStoreError::WriteIOError(0, e))?;
            f.sync_all()
                .map_err(|e| super::BackingStoreError::WriteIOError(0, e))?;
        }
        Ok(())
    }

    fn read_page(&mut self, page_id: u128) -> Result<Page, super::BackingStoreError> {
        self.open_file()?;
        let offset = self
            .page_index
            .get(&page_id)
            .ok_or(super::BackingStoreError::PageNotFound(page_id as usize))?;
        let f = self.file.as_mut().unwrap();
        f.seek(SeekFrom::Start(*offset))
            .map_err(|e| super::BackingStoreError::ReadIOError(0, e))?;
        let mut len_buf = [0u8; 8];
        f.read_exact(&mut len_buf)
            .map_err(|e| super::BackingStoreError::ReadIOError(0, e))?;
        let len = u64::from_be_bytes(len_buf) as usize;
        let mut buf = vec![0u8; len];
        f.read_exact(&mut buf)
            .map_err(|e| super::BackingStoreError::ReadIOError(0, e))?;
        let page = try_from_slice::<Page>(&buf)
            .map_err(|e| super::BackingStoreError::DecodeError(0, e))?;
        Ok(page)
    }

    fn write_page(&mut self, page: Page) -> Result<(), super::BackingStoreError> {
        self.open_file()?;
        let bytes = try_into_vec(&page).map_err(|e| super::BackingStoreError::EncodeError(0, e))?;
        let len = bytes.len() as u64;
        let f = self.file.as_mut().unwrap();
        let end_offset = f
            .seek(SeekFrom::End(0))
            .map_err(|e| super::BackingStoreError::WriteIOError(0, e))?;
        let len_be = (len as u64).to_be_bytes();
        f.write_all(&len_be)
            .map_err(|e| super::BackingStoreError::WriteIOError(0, e))?;
        f.write_all(&bytes)
            .map_err(|e| super::BackingStoreError::WriteIOError(0, e))?;
        f.flush()
            .map_err(|e| super::BackingStoreError::WriteIOError(0, e))?;
        self.page_index.insert(page.page_id(), end_offset);
        Ok(())
    }

    fn allocate_page(&mut self) -> Result<u128, super::BackingStoreError> {
        let allocated = self.next_page_id;
        self.next_page_id = self
            .next_page_id
            .checked_add(1)
            .ok_or(super::BackingStoreError::OutOfSpace)?;
        Ok(allocated)
    }

    fn free_page(&mut self, page_id: u128) -> Result<(), super::BackingStoreError> {
        self.page_index.remove(&page_id);
        Ok(())
    }

    fn total_pages(&self) -> usize {
        self.page_index.len()
    }

    fn write_journal_entry(
        &mut self,
        _entry: epiloglite_core::JournalEntry,
    ) -> Result<(), super::BackingStoreError> {
        Ok(())
    }

    fn page_size(&self) -> usize {
        1usize << self.header.page_size_exponent()
    }
}
