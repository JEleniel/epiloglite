use bincode::{
    de::read::Reader,
    enc::write::Writer,
    error::{DecodeError, EncodeError},
};
use serde::{Deserialize, Serialize};

use crate::{BINCODE_CONFIG, CInt};

/// An entry in the Journal
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JournalEntry {
    /// The beginning of a transaction
    BeginTransaction {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The transaction id
        transaction_id: CInt,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// The commit of a transaction
    CommitTransaction {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The transaction id
        transaction_id: CInt,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// The rollback of a transaction
    RollbackTransaction {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The transaction id
        transaction_id: CInt,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// A create table operation
    CreateTable {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The table id
        table_id: CInt,
    },
    /// A create index operation
    CreateIndex {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The index id
        index_id: CInt,
    },
    /// A create view operation
    CreateView {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The view id
        view_id: CInt,
    },
    /// An alter table operation
    AlterTable {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// Is this the before action record (false) or the after action record (true)
        after: bool,
        /// The table id
        table_id: CInt,
        /// The serialized table definition
        table_def: Vec<u8>,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// A drop table operation
    DropTable {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The table id
        table_id: CInt,
        /// The serialized table definition
        table_def: Vec<u8>,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// A drop index operation
    DropIndex {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The index id
        index_id: CInt,
        /// The serialized index definition
        index_def: Vec<u8>,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// A drop view operation
    DropView {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The view id
        view_id: CInt,
        /// The serialized view definition
        view_def: Vec<u8>,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// An insert operation
    Insert {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The table id
        table_id: CInt,
        /// The row id
        row_id: CInt,
        /// The serialized row data
        row_data: Vec<u8>,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// An update operation
    Update {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// Is this the after image (true) or before image (false)
        after: bool,
        /// Is this an upsert operation
        upsert: bool,
        /// The table id
        table_id: CInt,
        /// The row id
        row_id: CInt,
        /// The serialized row data
        row_data: Vec<u8>,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
    /// A delete operation
    Delete {
        /// The timestamp of the operation
        timestamp: chrono::DateTime<chrono::Utc>,
        /// The table id
        table_id: CInt,
        /// The row id
        row_id: CInt,
        /// The serialized row data
        row_data: Vec<u8>,
        /// The CRC32 checksum of the entry
        crc: u32,
    },
}

impl JournalEntry {
    /// Deserialize a JournalEntry from a reader
    pub fn try_from_reader<T: Reader>(reader: &mut T) -> Result<Self, DecodeError> {
        bincode::serde::decode_from_reader(reader, BINCODE_CONFIG)
    }

    /// Serialize the JournalEntry to a writer
    pub fn try_to_writer<T: Writer>(&self, writer: &mut T) -> Result<(), EncodeError> {
        bincode::serde::encode_into_writer(self, writer, BINCODE_CONFIG)
    }
}

impl TryInto<Vec<u8>> for &JournalEntry {
    type Error = EncodeError;

    /// Serialize the JournalEntry to a byte vector
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serde::encode_to_vec(self, BINCODE_CONFIG)
    }
}

impl TryFrom<&[u8]> for JournalEntry {
    type Error = DecodeError;

    /// Deserialize a JournalEntry from a byte slice
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let (entry, _): (JournalEntry, _) =
            bincode::serde::decode_from_slice(value, BINCODE_CONFIG)?;
        Ok(entry)
    }
}
