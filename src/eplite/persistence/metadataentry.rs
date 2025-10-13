use serde::{Deserialize, Serialize};

use crate::{CInt, persistence::OffsetPointer};

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
