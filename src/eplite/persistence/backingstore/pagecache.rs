use std::collections::HashMap;

use epiloglite_core::{CInt, PageFlags};
use tokio::sync::RwLock;

use crate::eplite::persistence::backingstore::{BackingStore, Page, PagerError};

#[derive(Debug)]
struct PageCache {
    max_pages: usize,
    backing_store: Box<dyn BackingStore>,
    system_info: SystemInfo,
    cache: RwLock<HashMap<CInt, CachedPage>>,
}

impl PageCache {
    /// Create a new PageCache with a suggested maximum number of pages.
    pub fn new(backing_store: Box<dyn BackingStore>, suggested_max_pages: usize) -> Self {
        // Default to the lesser of suggested max pages and 20% of available memory
        let system_info = SystemInfo::new();
        let page_size = backing_store.page_size();
        let max_pages = std::cmp::min(
            suggested_max_pages,
            (system_info.available_memory_bytes * 0.2) / page_size as usize,
        );

        Self {
            backing_store,
            cache: RwLock::new(HashMap::new()),
            system_info,
            max_pages,
        }
    }

    /// Get a page from the cache or load it from the backing store if not present.
    pub fn get_page(&mut self, page_id: CInt) -> Result<&Page, PagerError> {
        if self.cache.contains_key(&page_id) {
            let cached_page = self.cache.get_mut().unwrap().get_mut(&page_id).unwrap();
            cached_page.last_accessed = chrono::Utc::now();
            return Ok(&cached_page.page);
        } else {
            let new_page = self.backing_store.load_page(page_id).ok()?;
            self.push(new_page).ok()?;
            return Ok(&new_page);
        }
    }

    pub fn allocate(&mut self, container_id: CInt) -> Result<CInt, PagerError> {
        let new_page_id = self.backing_store.allocate_page()?;
        let new_page = Page::new(new_page_id, self.backing_store.page_size());
        self.push(new_page)?;
        Ok(new_page_id)
    }

    /// Push a page into the cache, evicting if necessary.
    fn push(&mut self, page: Page) -> Result<(), PagerError> {
        if self.cache.contains_key(page.page_id()) {
            self.cache[page.page_id()].last_accessed = chrono::Utc::now();
            self.cache[page.page_id()].page = page;
            return Ok(());
        }
        self.try_evict();
        let cached_page = CachedPage {
            last_accessed: chrono::Utc::now(),
            page,
        };
        self.cache.insert(page.page_id(), cached_page);
    }

    /// Try to evict a page; if unable, flush dirty pages and try again.
    /// This should never fail unless the backing store is unable to flush.
    fn try_evict(&mut self) -> Result<(), PagerError> {
        if Ok(e) = self.evict_if_needed() {
            return Ok(());
        } else {
            self.flush()?;
            self.evict_if_needed()
        }
    }

    fn evict_if_needed(&mut self) -> Result<(), PagerError> {
        if self.cache.len() >= self.max_size {
            if let Some((&oldest_page_id, _)) = self
                .cache
                .iter()
                .filter(|(.., cached_page)| !cached_page.page.flags().contains(PageFlags::DIRTY))
                .min_by_key(|(_, cached_page)| cached_page.last_accessed)
            {
                self.cache.remove(&oldest_page_id);
            } else {
                return Err(PagerError::CacheEvictionFailed);
            }
        }
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), PagerError> {
        for (_, cached_page) in self.cache.iter_mut() {
            if cached_page.page.flags().contains(PageFlags::DIRTY) {
                self.backing_store.save_page(&cached_page.page)?;
                cached_page.page.clear_flags(PageFlags::DIRTY);
            }
        }
        Ok(())
    }
}

/// A cached page with its last accessed timestamp.
#[derive(Debug)]
struct CachedPage {
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub page: Page,
}
