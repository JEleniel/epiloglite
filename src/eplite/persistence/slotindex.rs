use crate::RecordFlags;
use epiloglite_core::CInt;
use epiloglite_derive::collection;
use flagset::FlagSet;
use serde::{Deserialize, Serialize};

/// Local slot index entry mapping a row ID to its offset within a page
#[collection]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlotIndexEntry {
    pub record_id: CInt,
    pub offset: usize,
}

pub use SlotIndexEntry as SlotIndexEntry_Collection;
