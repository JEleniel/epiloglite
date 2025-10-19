use epiloglite_core::CInt;
use flagset::{FlagSet, flags};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotIndexEntry {
    pub flags: FlagSet<SlotFlags>,
    pub record_id: CInt,
    pub offset: CInt,
    pub length: CInt,
}

flags! {
    /// Flags for slot index entries
    pub enum SlotFlags: u8 {
        /// Slot is free
        Free = 0b00000001,
        /// Slot contains an active record
        Active = 0b00000010,
        /// Slot contains a dirty record
        Dirty = 0b00000100,
    }
}
