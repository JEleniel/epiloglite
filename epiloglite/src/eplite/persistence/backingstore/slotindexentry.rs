use epiloglite_core::Cu128;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SlotIndexEntry {
    pub slot_index_entry_type: SlotIndexEntryType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum SlotIndexEntryType {
    Free {
        offset: Cu128,
        length: Cu128,
    },
    Used {
        record_id: Cu128,
        offset: Cu128,
        length: Cu128,
    },
}

impl SlotIndexEntry {
    /// Create a new used slot entry
    pub fn new_used(record_id: Cu128, offset: Cu128, length: Cu128) -> Self {
        SlotIndexEntry {
            slot_index_entry_type: SlotIndexEntryType::Used {
                record_id,
                offset,
                length,
            },
        }
    }

    /// Create a new free slot entry
    pub fn new_free(offset: Cu128, length: Cu128) -> Self {
        SlotIndexEntry {
            slot_index_entry_type: SlotIndexEntryType::Free { offset, length },
        }
    }

    /// If this slot is used, return (record_id, offset, length)
    pub fn as_used(&self) -> Option<(Cu128, Cu128, Cu128)> {
        match &self.slot_index_entry_type {
            SlotIndexEntryType::Used {
                record_id,
                offset,
                length,
            } => Some((record_id.clone(), offset.clone(), length.clone())),
            _ => None,
        }
    }

    /// If this slot is free, return (offset, length)
    pub fn as_free(&self) -> Option<(Cu128, Cu128)> {
        match &self.slot_index_entry_type {
            SlotIndexEntryType::Free { offset, length } => Some((offset.clone(), length.clone())),
            _ => None,
        }
    }

    /// Set as used
    pub fn set_used(&mut self, record_id: Cu128, offset: Cu128, length: Cu128) {
        self.slot_index_entry_type = SlotIndexEntryType::Used {
            record_id,
            offset,
            length,
        };
    }

    /// Set as free
    pub fn set_free(&mut self, offset: Cu128, length: Cu128) {
        self.slot_index_entry_type = SlotIndexEntryType::Free { offset, length };
    }
}
