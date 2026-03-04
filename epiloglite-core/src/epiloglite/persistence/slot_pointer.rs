use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotPointer {
    pub page_id: u128,
    pub offset: usize,
    pub length: usize,
}

impl SlotPointer {
    pub fn zero() -> Self {
        SlotPointer {
            page_id: 0,
            offset: 0,
            length: 0,
        }
    }
}
