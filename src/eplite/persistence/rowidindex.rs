use epiloglite_core::{CInt, OffsetPointer};
use serde::{Deserialize, Serialize};

/// RowID index mapping record IDs to their storage locations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RowIDIndexEntry {
    collection_id: CInt,
    row_id: CInt,
    location: OffsetPointer,
}
