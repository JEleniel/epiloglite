use epiloglite_core::{OffsetPointer, PageFlags, serialized_size};
use flagset::FlagSet;

use crate::{
    eplite::{
        JournalEntry,
        persistence::backingstore::{BackingStore, Page, PageHeader, page},
    },
    os::SystemInfo,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MemoryBackingStore {
    header: DatabaseHeader,
    freelist_pointer: OffsetPointer,
    metadata_pointer: OffsetPointer,
    journal_pointer: OffsetPointer,
    page_cache: PageCache,
    sys_info: SystemInfo,
}

impl MemoryBackingStore {
    pub fn new(application_id: CInt, migration_version: CInt) -> Self {
        let page_size_exponent = Self::calculate_page_size_exponent();
        let page_size: usize = 1usize << page_size_exponent;

        let mut page_cache = PageCache::new(1024); // Initial cache size of 1024 pages

        let database_header = DatabaseHeader::default()
            .with_application_id(application_id)
            .with_page_size_exponent(page_size_exponent)
            .with_migration_version(migration_version);

        let freelist: HashSet<usize> = HashSet::new();
        freelist.insert(3); // Page 3 is the first free page
        let freelist_pointer = OffsetPointer::new(0, 102); // Pointing to freelist on page 0

        let page_0: Page = Page::new(0.into(), 1.into(), page_size, FlagSet::empty());
        page_0.add_entry(&database_header);
        page_0.add_entry(vec![0u8; 101 - serialized_size(&database_header)]); // Padding to fill the page
        page_0.add_entry(&freelist);
        pages.insert(0, page_0);

        let metadata: HashMap<usize, OffsetPointer> = HashMap::new();
        metadata.insert(1, OffsetPointer::new(0, 102)); // Container 1 is the FreeList
        metadata.insert(2, OffsetPointer::new(1, 102)); // Container 2 is the Metadata
        let metadata_pointer = OffsetPointer::new(1, 102); // Pointing to metadata on page 1

        let page_1: Page = Page::new(1.into(), 2.into(), page_size, FlagSet::empty());
        page_1.add_entry(&database_header);
        page_1.add_entry(vec![0u8; 101 - serialized_size(&database_header)]); // Padding to fill the page
        page_1.add_entry(&metadata);
        pages.insert(1, page_1);

        let journal: Vec<JournalEntry> = Vec::new();
        let journal_pointer = OffsetPointer::new(2, 102); // Pointing to journal on page 2
        let page_2: Page = Page::new(2.into(), 3.into(), page_size, FlagSet::empty());
        page_2.add_entry(&journal);
        pages.insert(2, page_2);

        let mut page_3_header: PageHeader =
            PageHeader::new(page_size, 3.into(), 0.into(), PageFlags::FREE);

        let free_page = Page::new_free_page(3.into(), page_size);
        pages.insert(3, free_page);

        Self {
            header: database_header,
            freelist_pointer: freelist_pointer,
            metadata_pointer: metadata_pointer,
            journal_pointer: journal_pointer,
            page_cache,
            sys_info: SystemInfo::new(),
        }
    }

    /// Test allocating and writing pages from 2^9 to 2^30 bytes to determine the optimal page size.
    fn calculate_page_size_exponent() -> u8 {
        let mut best_exponent: (u8, Duration) = (0, Duration::MAX);
        for i in 9..=30 {
            let size: usize = 1usize << i;
            let start = chrono::Utc::now();
            let mut vec: Vec<u8> = Vec::with_capacity(size);
            for _ in 0..size {
                vec.push(0);
            }
            let duration = chrono::Utc::now() - start;

            if duration < best_exponent.1 {
                best_exponent = (i as u8, duration);
            }
        }
        best_exponent.0
    }
}

impl BackingStore for MemoryBackingStore {
    fn open(&mut self) -> Result<(), super::BackingStoreError> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), super::BackingStoreError> {
        todo!()
    }

    fn close(&mut self) -> Result<(), super::BackingStoreError> {
        todo!()
    }

    fn read_page(&mut self, page_id: usize) -> Result<Page, super::BackingStoreError> {
        todo!()
    }

    fn write_page(&mut self, page: Page) -> Result<(), super::BackingStoreError> {
        todo!()
    }

    fn allocate_page(&mut self) -> Result<Page, super::BackingStoreError> {
        todo!()
    }

    fn free_page(&mut self, page_id: usize) -> Result<(), super::BackingStoreError> {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn free_pages(&self) -> usize {
        todo!()
    }

    fn write_journal_entry(
        &mut self,
        entry: crate::eplite::JournalEntry,
    ) -> Result<(), super::BackingStoreError> {
        todo!()
    }
}
