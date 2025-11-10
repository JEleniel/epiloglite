//! Slot index for EpilogLite persistence layer
//!
//! A SlotIndex represents a small index entry mapping a record id to an
//! offset within a page. Historically the project used a different layout;
//! currently we use a compact, explicit struct here to satisfy module
//! consumers.

/// Simple slot index entry used by persistence modules.
///
/// Fields:
/// - `record_id`: global record identifier (u128)
/// - `offset`: byte offset within the page (u64)
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SlotIndex {
    pub record_id: u128,
    pub offset: u64,
}

impl SlotIndex {
    pub fn new(record_id: u128, offset: u64) -> Self {
        Self { record_id, offset }
    }
}
