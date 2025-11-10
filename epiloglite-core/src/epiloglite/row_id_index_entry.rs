use serde::{Deserialize, Serialize};

use crate::{Cu128, OffsetPointer};

/// RowID index mapping record IDs to their storage locations
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RowIDIndexEntry {
    collection_id: Cu128,
    row_id: Cu128,
    location: OffsetPointer,
}
