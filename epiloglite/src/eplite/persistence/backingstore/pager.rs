use std::sync::{Arc, Mutex};

use epiloglite_core::Cu128;

use crate::eplite::persistence::backingstore::{BackingStore, BackingStoreError, Page, PageCache};

#[derive(thiserror::Error, Debug)]
pub enum PagerError {
    #[error("Backing store error: {0}")]
    BackingStore(#[from] BackingStoreError),
    #[error("Invalid Cu128 conversion: {0}")]
    InvalidCu128(#[from] epiloglite_core::CIntError),
}

pub struct Pager<B>
where
    B: BackingStore + Send,
{
    backing_store: Arc<Mutex<B>>,
    cache_pages: usize,
    page_cache: PageCache<B>,
}

impl<B> Pager<B>
where
    B: BackingStore + Send,
{
    pub fn new(backing_store: B, cache_pages: usize) -> Self {
        let shared = Arc::new(Mutex::new(backing_store));
        let page_cache = PageCache::new(shared.clone(), cache_pages);
        Self {
            backing_store: shared,
            cache_pages,
            page_cache,
        }
    }

    pub fn get_page(&self, page_id: Cu128) -> Result<Arc<Page>, PagerError> {
        self.page_cache.get_page(page_id).map_err(PagerError::from)
    }
}
