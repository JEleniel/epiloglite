use serde::{Deserialize, Serialize};

use crate::CInt;

/// A pointer to a specific location in a data file, identified by page number and offset within that page.
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct OffsetPointer {
    /// The page number in the data file.
    pub page_number: CInt,
    /// The offset within the page.
    pub offset: CInt,
}

impl OffsetPointer {
    /// Create a null pointer (both page number and offset are zero).
    pub fn null() -> Self {
        Self {
            page_number: CInt::zero(),
            offset: CInt::zero(),
        }
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8);
        bytes.extend(&self.page_number.bytes());
        bytes.extend(&self.offset.bytes());
        bytes
    }
}

#[cfg(test)]
#[path = "tests/t_offsetpointer.rs"]
mod t_offsetpointer;
