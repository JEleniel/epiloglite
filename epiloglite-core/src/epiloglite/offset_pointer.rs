use crate::{Cu64, Cu128};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// A pointer to a specific location in a data file, identified by page number and offset within that page.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OffsetPointer {
    /// The page number in the data file.
    pub page_id: Cu128,
    /// The offset within the page.
    pub offset: Cu64,
}

impl OffsetPointer {
    /// Create a null pointer (both page number and offset are zero).
    pub fn null() -> Self {
        Self {
            page_id: Cu128::from(0u128),
            offset: Cu64::from(0u64),
        }
    }
}

impl Display for OffsetPointer {
    /// Display the OffsetPointer in the format "(page_id:offset)"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(p{}:o{})", self.page_id, self.offset)
    }
}
