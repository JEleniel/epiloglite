use std::collections::HashMap;

use epiloglite_core::CInt;
use tokio::sync::RwLock;

use crate::eplite::persistence::backingstore::BackingStore;

#[derive(Debug)]
pub struct Pager {
    backing_store: Box<dyn BackingStore>,
    cache_pages: usize,
    page_cache: PageCache,
}

impl Pager {
    pub fn new(backing_store: Box<dyn BackingStore>, page_size: usize, cache_pages: usize) -> Self {
        Self {
            backing_store,
            total_pages,
            page_cache: PageCache::new(total_pages),
        }
    }

    pub fn get_page(&mut self, page_id: CInt) -> Result<&Page, PagerError> {
        self.page_cache
            .get_page(page_id)
            .ok_or(PagerError::PageNotFound(page_id))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PagerError {
    #[error("Page not found: {0}")]
    PageNotFound(CInt),
    #[error("Page cache is full and eviction failed")]
    CacheEvictionFailed,
}
