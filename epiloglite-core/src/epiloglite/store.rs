use std::collections::HashMap;

use serde::Serialize;

use crate::{Container, Record, StoreHeader};

#[derive(Debug, Clone, Serialize)]
pub struct Store<T>
where
    T: Record + serde::Serialize + serde::de::DeserializeOwned + Clone + std::fmt::Debug,
{
    pub containers: HashMap<u128, Container<T>>,
    pub header: Container<StoreHeader>,
    pub free_page_list: Vec<u128>,
    // Persistence-specific metadata/index structures live in the top-level
    // `epiloglite` crate. Core `Store` should not reference those types
    // directly to avoid cross-crate import cycles.
    pub metadata_index: HashMap<u128, u128>,
    pub row_id_index: HashMap<u128, u128>,
}
