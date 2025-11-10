use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::eplite::persistence::backingstore::PageFlags;

use crate::eplite::persistence::backingstore::PagerError;
use crate::eplite::persistence::backingstore::{BackingStore, Page};
use epiloglite_core::Cu128;

/// A cache of pages backed by an underlying backing store `B`.
pub struct PageCache<B>
where
    B: BackingStore + Send,
{
    max_pages: usize,
    backing_store: Arc<Mutex<B>>,
    cache: Mutex<HashMap<Cu128, CachedPage>>,
}

impl<B> PageCache<B>
where
    B: BackingStore + Send,
{
    /// Create a new PageCache with a suggested maximum number of pages.
    /// Accepts a shared Arc<Mutex<B>> backing store for dependency injection.
    pub fn new(backing_store: Arc<Mutex<B>>, suggested_max_pages: usize) -> Self {
        // Simpler heuristic: prefer the suggested value but cap it to a reasonable
        // maximum to avoid excessive memory use. Previously this used a SystemInfo
        // shim to estimate available memory; that shim was removed in favor of
        // a conservative cap here.
        let page_size = {
            let bs = backing_store.lock().unwrap();
            bs.page_size()
        };

        // Cap max pages to either the suggested value or a conservative default
        // based on page size (here: up to 16k pages unless suggested is lower).
        let conservative_cap = std::cmp::max(1024usize, 16_384usize.saturating_sub(page_size));
        let max_pages = std::cmp::min(suggested_max_pages, conservative_cap);

        Self {
            backing_store,
            cache: Mutex::new(HashMap::new()),
            max_pages,
        }
    }

    /// Get a page from the cache or load it from the backing store if not present.
    /// Only fails if the underlying backing store fails to read the page.
    pub fn get_page(&self, page_id: Cu128) -> Result<std::sync::Arc<Page>, PagerError> {
        // Fast-path: check cache
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get_mut(&page_id) {
                cached.last_accessed = chrono::Utc::now();
                return Ok(Arc::new(cached.page.clone()));
            }
        }

        // Not present: read from backing store. Only fail if backing store can't provide the page.
        // Not in cache: convert CInt page id into canonical u128 for the backing store
        let page = {
            let pid_u128: u128 = std::convert::TryFrom::try_from(page_id.clone())?;
            let mut bs = self.backing_store.lock().unwrap();
            bs.read_page(pid_u128).map_err(PagerError::from)?
        };

        // Push into cache (evict if required)
        self.push(page.clone())?;
        Ok(Arc::new(page))
    }

    /// Allocate a new page in the backing store and return its id.
    pub fn allocate(&self) -> Result<u128, PagerError> {
        let new_page_id = {
            let mut bs = self.backing_store.lock().unwrap();
            bs.allocate_page().map_err(PagerError::from)?
        };
        Ok(new_page_id)
    }

    /// Push a page into the cache, evicting if necessary.
    fn push(&self, page: Page) -> Result<(), PagerError> {
        let mut cache = self.cache.lock().unwrap();
        let page_key: Cu128 = page.page_id();
        if cache.contains_key(&page_key) {
            if let Some(entry) = cache.get_mut(&page_key) {
                entry.last_accessed = chrono::Utc::now();
                entry.page = page;
            }
            return Ok(());
        }

        // Ensure there's room
        if cache.len() >= self.max_pages {
            // Try to evict aged, unmodified pages first
            if !self.evict_aged_unmodified(&mut cache) {
                // If couldn't evict, flush dirty pages and then evict the oldest page regardless of dirty flag.
                self.flush()?;
                // Evict oldest page regardless
                if let Some(oldest_key) = cache
                    .iter()
                    .min_by_key(|(_, c)| c.last_accessed)
                    .map(|(k, _)| k.clone())
                {
                    cache.remove(&oldest_key);
                }
            }
        }

        let cached_page = CachedPage {
            last_accessed: chrono::Utc::now(),
            page,
        };
        cache.insert(cached_page.page.page_id(), cached_page);
        Ok(())
    }

    /// Evict the least recently used unmodified page. Returns true if eviction happened.
    fn evict_aged_unmodified(&self, cache: &mut HashMap<Cu128, CachedPage>) -> bool {
        if cache.is_empty() {
            return false;
        }

        if let Some(evict_id) = cache
            .iter()
            .filter(|(_, cached)| !cached.page.flags().contains(PageFlags::Dirty))
            .min_by_key(|(_, cached)| cached.last_accessed)
            .map(|(k, _)| k.clone())
        {
            cache.remove(&evict_id);
            true
        } else {
            false
        }
    }

    /// Flush dirty pages to the backing store. Writes aged, dirty pages first.
    pub fn flush(&self) -> Result<(), PagerError> {
        // Collect pages to flush ordered by last_accessed (oldest first)
        let mut to_flush: Vec<Page> = Vec::new();
        {
            let cache = self.cache.lock().unwrap();
            for (_, cached) in cache.iter() {
                if cached.page.flags().contains(PageFlags::Dirty) {
                    to_flush.push(cached.page.clone());
                }
            }
        }

        // sort by last_accessed ascending -> aged first
        to_flush.sort_by_key(|p| {
            // Need to map page id back to timestamp; look up in cache
            let cache = self.cache.lock().unwrap();
            cache
                .get(&p.page_id())
                .map(|c| c.last_accessed)
                .unwrap_or(chrono::Utc::now())
        });

        // Write pages to backing store
        let mut bs = self.backing_store.lock().unwrap();
        for page in to_flush.into_iter() {
            bs.write_page(page.clone()).map_err(PagerError::from)?;
            // Clear DIRTY flag in cache
            let mut cache = self.cache.lock().unwrap();
            if let Some(entry) = cache.get_mut(&page.page_id()) {
                entry.page.set_clean();
            }
        }

        Ok(())
    }
}

/// A cached page with its last accessed timestamp.
#[derive(Debug, Clone)]
struct CachedPage {
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub page: Page,
}
