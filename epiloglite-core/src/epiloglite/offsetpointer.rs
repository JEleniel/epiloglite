use serde::{Deserialize, Serialize};
use std::fmt::Display;

// Use plain u128 for offsets and page identifiers instead of the CInt wrapper.

/// A pointer to a specific location in a data file, identified by page number and offset within that page.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OffsetPointer {
    /// The page number in the data file.
    pub page_id: u128,
    /// The offset within the page.
    pub offset: u128,
}

impl OffsetPointer {
    /// Create a null pointer (both page number and offset are zero).
    pub fn null() -> Self {
        Self {
            page_id: 0u128,
            offset: 0u128,
        }
    }
}

impl Display for OffsetPointer {
    /// Display the OffsetPointer in the format "(page_id:offset)"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.page_id, self.offset)
    }
}
