use std::fmt::Display;

use bincode::{
    de::read::Reader,
    enc::write::Writer,
    error::{DecodeError, EncodeError},
};
use serde::{Deserialize, Serialize};

use crate::{BINCODE_CONFIG, CInt};

/// A pointer to a specific location in a data file, identified by page number and offset within that page.
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize)]
pub struct OffsetPointer {
    /// The page number in the data file.
    pub page_id: CInt,
    /// The offset within the page.
    pub offset: CInt,
}

impl OffsetPointer {
    /// Create a null pointer (both page number and offset are zero).
    pub fn null() -> Self {
        Self {
            page_id: CInt::zero(),
            offset: CInt::zero(),
        }
    }

    /// Deserialize a OffsetPointer from a reader
    pub fn try_from_reader<T: Reader>(reader: &mut T) -> Result<Self, DecodeError> {
        bincode::serde::decode_from_reader(reader, BINCODE_CONFIG)
    }

    /// Serialize the OffsetPointer to a writer
    pub fn try_to_writer<T: Writer>(&self, writer: &mut T) -> Result<(), EncodeError> {
        bincode::serde::encode_into_writer(self, writer, BINCODE_CONFIG)
    }
}

impl Display for OffsetPointer {
    /// Display the OffsetPointer in the format "(page_id:offset)"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{})", self.page_id, self.offset)
    }
}

impl TryInto<Vec<u8>> for &OffsetPointer {
    type Error = EncodeError;

    /// Serialize the OffsetPointer to a byte vector
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serde::encode_to_vec(self, BINCODE_CONFIG)
    }
}

impl TryFrom<&[u8]> for OffsetPointer {
    type Error = DecodeError;

    /// Deserialize a OffsetPointer from a byte slice
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (entry, _): (OffsetPointer, _) =
            bincode::serde::decode_from_slice(value, BINCODE_CONFIG)?;
        Ok(entry)
    }
}
