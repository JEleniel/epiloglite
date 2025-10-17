use epiloglite_core::{CInt, OffsetPointer, RecordFlags};
use epiloglite_derive::collection;
use flagset::FlagSet;
use serde::{Deserialize, Serialize};

/// RowID index mapping record IDs to their storage locations
#[collection]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RowIDIndex {
    collection_id: CInt,
    row_id: CInt,
    location: OffsetPointer,
}
