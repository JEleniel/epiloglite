use bincode::{
    de::read::Reader,
    enc::write::Writer,
    error::{DecodeError, EncodeError},
};
use serde::{Deserialize, Serialize};

use crate::{BINCODE_CONFIG, CInt};
use epiloglite_core::OffsetPointer;

/// Represents a metadata entry in the database, a.k.a. a *_def
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetadataEntry {
    /// Represents a table definition
    Table {
        /// Unique identifier for the table
        table_id: CInt,
        /// Name of the table
        name: String,
        /// Pointer to the first page of the table's data
        first_page: OffsetPointer,
        /// Checksum for data integrity verification
        crc: u32,
    },
    /// Represents an index definition
    Index {
        /// Unique identifier for the index
        index_id: CInt,
        /// Identifier of the table this index belongs to
        table_id: CInt,
        /// Name of the index
        name: String,
        /// Serialized index definition
        index_def: Vec<u8>,
        /// Checksum for data integrity verification
        crc: u32,
    },
    /// Represents a view definition
    View {
        /// Unique identifier for the view
        view_id: CInt,
        /// Name of the view
        name: String,
        /// Serialized view definition
        view_def: Vec<u8>,
        /// Checksum for data integrity verification
        crc: u32,
    },
}

impl MetadataEntry {
    /// Deserialize a MetadataEntry from a reader
    pub fn try_from_reader<T: Reader>(reader: &mut T) -> Result<Self, DecodeError> {
        bincode::serde::decode_from_reader(reader, BINCODE_CONFIG)
    }

    /// Serialize the MetadataEntry to a writer
    pub fn try_to_writer<T: Writer>(&self, writer: &mut T) -> Result<(), EncodeError> {
        bincode::serde::encode_into_writer(self, writer, BINCODE_CONFIG)
    }
}

impl TryInto<Vec<u8>> for &MetadataEntry {
    type Error = EncodeError;

    /// Serialize the MetadataEntry to a byte vector
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serde::encode_to_vec(self, BINCODE_CONFIG)
    }
}

impl TryFrom<&[u8]> for MetadataEntry {
    type Error = DecodeError;

    /// Deserialize a MetadataEntry from a byte slice
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (entry, _): (MetadataEntry, _) =
            bincode::serde::decode_from_slice(value, BINCODE_CONFIG)?;
        Ok(entry)
    }
}
