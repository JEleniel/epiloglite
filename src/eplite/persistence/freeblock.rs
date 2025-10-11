use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreeBlock {
    next_free_block: u32,
    num_free_bytes: u32,
}
