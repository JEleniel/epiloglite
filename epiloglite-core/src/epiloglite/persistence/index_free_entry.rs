use serde::{Deserialize, Serialize};

use crate::SlotPointer;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexFreeEntry {
    pub collection_id: u128,
    pub slot_pointer: SlotPointer,
}
