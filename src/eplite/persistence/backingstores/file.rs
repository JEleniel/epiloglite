//! File-backed backing store implementation.

use chrono::{Duration, TimeDelta};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions, create_dir_all},
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use crate::eplite::persistence::{
    DatabaseHeader, Page,
    backingstores::{BackingStore, BackingStoreError, file},
};
use epiloglite_core::{CInt, calculate_crc, try_from_slice, try_into_vec};

/// File-backed backing store
pub struct FileBackingStore {
    file_path: PathBuf,
    application_id: CInt,
    migration_version: CInt,
    page_cache_size: usize,
    create: bool,
    test_large_pages: bool,
    file: Option<File>,
    handle_timeout: chrono::Duration,
    page_size: usize,
    total_pages: usize,
    header: DatabaseHeader,
    page_cache: BTreeMap<usize, Vec<u8>>,
}

impl FileBackingStore {
    /// Create a new file-backed store, though not yet opened
    /// The application_id and migration_version will be used to verify existing databases and
    ///     added to new databases.
    /// Note: The page_cache_size is a hint for how many pages to keep in memory for fast access.
    ///     The actual number of cached pages may be lower or higher depending on available memory.
    /// Note: test_large_pages, if true, will test page sizes up to 2^63 bytes and may take a while.
    ///     It should not be used unless you expect to use very large pages.
    /// Note: if `create` is true the entire file path will be created (`mkdir -p ...`) if it does
    ///     not already exist
    pub fn new<P: Into<PathBuf>>(
        path: P,
        application_id: CInt,
        migration_version: CInt,
        page_cache_size: usize,
        create: bool,
        test_large_pages: bool,
    ) -> Result<Self, BackingStoreError> {
        let file_path: PathBuf = path.into();
        if !fs::exists(&file_path).unwrap() && !create {
            return Err(BackingStoreError::FileNotFound(file_path.clone()));
        } else if !fs::exists(&file_path).unwrap() {
            Self::create_database_file(
                &file_path,
                &application_id,
                &migration_version,
                test_large_pages,
            )?;
        }
        Ok(Self {
            application_id,
            create,
            file_path,
            file: None,
            handle_timeout: chrono::Duration::seconds(10),
            header: DatabaseHeader::default(), // Will be set on open
            migration_version,
            page_cache_size,
            page_cache: BTreeMap::new(),
            page_size: 0,
            test_large_pages,
            total_pages: 0,
        })
    }

    /// Perform a quick benchmark to determine the best page size for this system
    /// If `test_large_pages` is true, test up to 2^63 bytes pages (may take a while)
    /// Otherwise, test up to 2^15 bytes pages (32KB)
    fn calculate_page_size(path: &PathBuf, test_large_pages: bool) -> u8 {
        let mut test_file = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        let mut fastest_write_exponent: (u8, Duration) = (0, TimeDelta::MAX);
        for page_exponent in (9 as u8)..=(if test_large_pages { 63 } else { 15 }) {
            let page_size: usize = (1 as usize) << page_exponent;
            let test_page: Vec<u8> = vec![0b10101010; page_size];
            let mut average_duration: Duration = Duration::zero();
            for _ in 0..100 {
                let start = chrono::Utc::now();
                test_file.write_all(&test_page).unwrap();
                test_file.flush().unwrap();
                let duration = chrono::Utc::now() - start;
                average_duration += duration / 100;
            }
            if average_duration < fastest_write_exponent.1 {
                fastest_write_exponent = (page_exponent, average_duration);
            }
        }

        test_file.seek(SeekFrom::Start(0)).unwrap();
        let mut fastest_read_exponent: (u8, Duration) = (0, TimeDelta::MAX);
        for page_exponent in (9 as u8)..=(if test_large_pages { 63 } else { 15 }) {
            let page_size: usize = (1 as usize) << page_exponent;
            let mut test_page: Vec<u8> = vec![0b10101010; page_size];
            let mut average_duration = Duration::zero();
            for _ in 0..100 {
                let start = chrono::Utc::now();
                test_file.read_exact(&mut test_page).unwrap();
                let duration = chrono::Utc::now() - start;
                average_duration += duration / 100;
            }
            if average_duration < fastest_read_exponent.1 {
                fastest_read_exponent = (page_exponent, average_duration);
            }
        }

        let choice = fastest_read_exponent.1 * 80 - fastest_write_exponent.1 * 20;
        if choice >= Duration::zero() {
            fastest_read_exponent.0
        } else {
            fastest_write_exponent.0
        }
    }

    /// Create a new database file with the specified parameters
    /// This will create any necessary parent directories as well.
    fn create_database_file(
        file_path: &PathBuf,
        application_id: &CInt,
        migration_version: &CInt,
        test_large_pages: bool,
    ) -> Result<(), BackingStoreError> {
        // This check should never pass, but it is included for safety
        if fs::exists(file_path).unwrap() {
            return Err(BackingStoreError::FileExists(file_path.clone()));
        }
        create_dir_all(file_path.parent().unwrap()).unwrap();
        let page_size_exponent: u8 = Self::calculate_page_size(file_path, test_large_pages);
        let header: &mut DatabaseHeader = DatabaseHeader::default()
            .with_application_id(application_id)
            .with_page_size_exponent(page_size_exponent)
            .with_migration_version(migration_version);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(|e| BackingStoreError::IoError(e))?;
        let page0 = Page::<DatabaseHeader>::new(0.into(), 0.into(), header.page_size());
        page0.add_entry(header);
        let page0_bytes = try_into_vec(&page0).unwrap();
        file.write_all(&page0_bytes)?;
        file.write_all(&page0_bytes)?;
        file.flush()?;
        file.sync_all()?;
        Ok(())
    }
}

impl BackingStore for FileBackingStore {
    fn open(&mut self) -> Result<(), BackingStoreError> {
        Ok(())
    }

    fn flush(&mut self) -> Result<(), BackingStoreError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), BackingStoreError> {
        Ok(())
    }

    fn write_journal_entry(
        &mut self,
        _entry: crate::eplite::JournalEntry,
    ) -> Result<(), BackingStoreError> {
        Ok(())
    }
    fn read_page<T>(&mut self, page_id: usize) -> Result<Page<T>, BackingStoreError>
    where
        T: Serialize + DeserializeOwned + Clone + std::fmt::Debug,
    {
        Ok(page)
    }

    fn write_page<T>(&mut self, _page: Page<T>) -> Result<(), BackingStoreError>
    where
        T: Serialize + DeserializeOwned + std::fmt::Debug + Clone,
    {
        // Not implemented
        Ok(())
    }

    fn allocate_page<T>(&mut self) -> Result<T, BackingStoreError>
    where
        T: Serialize + DeserializeOwned + std::fmt::Debug + Clone,
    {
        // Not implemented
        Err(BackingStoreError::OutOfSpace)
    }

    fn free_page(&mut self, page_id: usize) -> Result<(), BackingStoreError> {
        Ok(())
    }

    fn total_pages(&self) -> usize {
        self.total_pages
    }

    fn free_pages(&self) -> usize {
        count
    }
}

// OS-specific notes:
// - File locking is not implemented; concurrent access may cause corruption, especially on Windows.
// - File::try_clone is not supported on all platforms and may not duplicate file position on Windows.
// - File permissions and atomicity of writes may differ on Unix vs Windows.
// - Large files (>2GB) may have issues on 32-bit systems.
