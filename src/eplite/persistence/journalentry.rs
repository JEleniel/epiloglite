use serde::{Deserialize, Serialize};

use crate::CInt;

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
    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        match self {
            JournalEntry::BeginTransaction {
                timestamp,
                transaction_id,
                crc,
            } => {
                bytes.push(1); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&transaction_id.bytes());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::CommitTransaction {
                timestamp,
                transaction_id,
                crc,
            } => {
                bytes.push(2); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&transaction_id.bytes());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::RollbackTransaction {
                timestamp,
                transaction_id,
                crc,
            } => {
                bytes.push(3); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&transaction_id.bytes());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::CreateTable {
                timestamp,
                table_id,
            } => {
                bytes.push(4); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&table_id.bytes());
            }
            JournalEntry::CreateIndex {
                timestamp,
                index_id,
            } => {
                bytes.push(5); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&index_id.bytes());
            }
            JournalEntry::CreateView { timestamp, view_id } => {
                bytes.push(6); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&view_id.bytes());
            }
            JournalEntry::AlterTable {
                timestamp,
                after,
                table_id,
                table_def,
                crc,
            } => {
                bytes.push(7); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.push(if *after { 1 } else { 0 });
                bytes.extend(&table_id.bytes());
                bytes.extend(table_def.clone());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::DropTable {
                timestamp,
                table_id,
                table_def,
                crc,
            } => {
                bytes.push(8); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&table_id.bytes());
                bytes.extend(table_def.clone());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::DropIndex {
                timestamp,
                index_id,
                index_def,
                crc,
            } => {
                bytes.push(9); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&index_id.bytes());
                bytes.extend(index_def.clone());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::DropView {
                timestamp,
                view_id,
                view_def,
                crc,
            } => {
                bytes.push(10); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&view_id.bytes());
                bytes.extend(view_def.clone());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::Insert {
                timestamp,
                table_id,
                row_id,
                row_data,
                crc,
            } => {
                bytes.push(11); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&table_id.bytes());
                bytes.extend(&row_id.bytes());
                bytes.extend(row_data.clone());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::Update {
                timestamp,
                after,
                upsert,
                table_id,
                row_id,
                row_data,
                crc,
            } => {
                bytes.push(12); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.push(if *after { 1 } else { 0 });
                bytes.push(if *upsert { 1 } else { 0 });
                bytes.extend(&table_id.bytes());
                bytes.extend(&row_id.bytes());
                bytes.extend(row_data.clone());
                bytes.extend(&crc.to_be_bytes());
            }
            JournalEntry::Delete {
                timestamp,
                table_id,
                row_id,
                row_data,
                crc,
            } => {
                bytes.push(13); // Entry type
                bytes.extend(timestamp.timestamp_millis().to_be_bytes());
                bytes.extend(&table_id.bytes());
                bytes.extend(&row_id.bytes());
                bytes.extend(row_data.clone());
                bytes.extend(&crc.to_be_bytes());
            }
        }
        bytes
    }

    /// Reconstruct a JournalEntry from a byte slice (reverse of bytes())
    pub fn from_bytes(mut data: &[u8]) -> Result<Self, String> {
        use crate::CInt;
        use std::convert::TryFrom;
        if data.is_empty() {
            return Err("Empty data".to_string());
        }
        let entry_type = data[0];
        data = &data[1..];
        // Helper to read i64 timestamp
        fn read_i64(data: &mut &[u8]) -> Result<i64, String> {
            if data.len() < 8 {
                return Err("Not enough bytes for timestamp".to_string());
            }
            let mut arr = [0u8; 8];
            arr.copy_from_slice(&data[..8]);
            *data = &data[8..];
            Ok(i64::from_be_bytes(arr))
        }
        // Helper to read u32
        fn read_u32(data: &mut &[u8]) -> Result<u32, String> {
            if data.len() < 4 {
                return Err("Not enough bytes for u32".to_string());
            }
            let mut arr = [0u8; 4];
            arr.copy_from_slice(&data[..4]);
            *data = &data[4..];
            Ok(u32::from_be_bytes(arr))
        }
        // Helper to read bool (1 byte)
        fn read_bool(data: &mut &[u8]) -> Result<bool, String> {
            if data.is_empty() {
                return Err("Not enough bytes for bool".to_string());
            }
            let b = data[0] != 0;
            *data = &data[1..];
            Ok(b)
        }
        // Helper to read CInt
        fn read_cint(data: &mut &[u8]) -> Result<CInt, String> {
            let v = data.to_vec();
            let c = CInt::try_from(&v[..]).map_err(|e| format!("CInt: {:?}", e))?;
            let len = c.bytes().len();
            *data = &data[len..];
            Ok(c)
        }
        // Helper to read Vec<u8> of known length
        fn read_vec(data: &mut &[u8], len: usize) -> Result<Vec<u8>, String> {
            if data.len() < len {
                return Err("Not enough bytes for Vec<u8>".to_string());
            }
            let v = data[..len].to_vec();
            *data = &data[len..];
            Ok(v)
        }
        // Helper to read remaining bytes
        fn read_rest(data: &mut &[u8]) -> Vec<u8> {
            let v = data.to_vec();
            *data = &[];
            v
        }
        // Use chrono for timestamp
        use chrono::TimeZone;
        // Helper for chrono 0.4+ (timestamp_millis_opt)
        fn chrono_from_millis(ms: i64) -> chrono::DateTime<chrono::Utc> {
            chrono::Utc.timestamp_millis_opt(ms).single().unwrap()
        }
        Ok(match entry_type {
            1 | 2 | 3 => {
                // Begin, Commit, Rollback Transaction
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let transaction_id = read_cint(&mut data)?;
                let crc = read_u32(&mut data)?;
                match entry_type {
                    1 => JournalEntry::BeginTransaction {
                        timestamp,
                        transaction_id,
                        crc,
                    },
                    2 => JournalEntry::CommitTransaction {
                        timestamp,
                        transaction_id,
                        crc,
                    },
                    3 => JournalEntry::RollbackTransaction {
                        timestamp,
                        transaction_id,
                        crc,
                    },
                    _ => unreachable!(),
                }
            }
            4 => {
                // CreateTable
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let table_id = read_cint(&mut data)?;
                JournalEntry::CreateTable {
                    timestamp,
                    table_id,
                }
            }
            5 => {
                // CreateIndex
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let index_id = read_cint(&mut data)?;
                JournalEntry::CreateIndex {
                    timestamp,
                    index_id,
                }
            }
            6 => {
                // CreateView
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let view_id = read_cint(&mut data)?;
                JournalEntry::CreateView { timestamp, view_id }
            }
            7 => {
                // AlterTable
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let after = read_bool(&mut data)?;
                let table_id = read_cint(&mut data)?;
                // Table def is all but last 4 bytes (crc)
                if data.len() < 4 {
                    return Err("Not enough bytes for AlterTable CRC".to_string());
                }
                let table_def = data[..data.len() - 4].to_vec();
                *&mut data = &data[data.len() - 4..];
                let crc = read_u32(&mut data)?;
                JournalEntry::AlterTable {
                    timestamp,
                    after,
                    table_id,
                    table_def,
                    crc,
                }
            }
            8 => {
                // DropTable
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let table_id = read_cint(&mut data)?;
                if data.len() < 4 {
                    return Err("Not enough bytes for DropTable CRC".to_string());
                }
                let table_def = data[..data.len() - 4].to_vec();
                *&mut data = &data[data.len() - 4..];
                let crc = read_u32(&mut data)?;
                JournalEntry::DropTable {
                    timestamp,
                    table_id,
                    table_def,
                    crc,
                }
            }
            9 => {
                // DropIndex
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let index_id = read_cint(&mut data)?;
                if data.len() < 4 {
                    return Err("Not enough bytes for DropIndex CRC".to_string());
                }
                let index_def = data[..data.len() - 4].to_vec();
                *&mut data = &data[data.len() - 4..];
                let crc = read_u32(&mut data)?;
                JournalEntry::DropIndex {
                    timestamp,
                    index_id,
                    index_def,
                    crc,
                }
            }
            10 => {
                // DropView
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let view_id = read_cint(&mut data)?;
                if data.len() < 4 {
                    return Err("Not enough bytes for DropView CRC".to_string());
                }
                let view_def = data[..data.len() - 4].to_vec();
                *&mut data = &data[data.len() - 4..];
                let crc = read_u32(&mut data)?;
                JournalEntry::DropView {
                    timestamp,
                    view_id,
                    view_def,
                    crc,
                }
            }
            11 => {
                // Insert
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let table_id = read_cint(&mut data)?;
                let row_id = read_cint(&mut data)?;
                if data.len() < 4 {
                    return Err("Not enough bytes for Insert CRC".to_string());
                }
                let row_data = data[..data.len() - 4].to_vec();
                *&mut data = &data[data.len() - 4..];
                let crc = read_u32(&mut data)?;
                JournalEntry::Insert {
                    timestamp,
                    table_id,
                    row_id,
                    row_data,
                    crc,
                }
            }
            12 => {
                // Update
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let after = read_bool(&mut data)?;
                let upsert = read_bool(&mut data)?;
                let table_id = read_cint(&mut data)?;
                let row_id = read_cint(&mut data)?;
                if data.len() < 4 {
                    return Err("Not enough bytes for Update CRC".to_string());
                }
                let row_data = data[..data.len() - 4].to_vec();
                *&mut data = &data[data.len() - 4..];
                let crc = read_u32(&mut data)?;
                JournalEntry::Update {
                    timestamp,
                    after,
                    upsert,
                    table_id,
                    row_id,
                    row_data,
                    crc,
                }
            }
            13 => {
                // Delete
                let timestamp = chrono_from_millis(read_i64(&mut data)?);
                let table_id = read_cint(&mut data)?;
                let row_id = read_cint(&mut data)?;
                if data.len() < 4 {
                    return Err("Not enough bytes for Delete CRC".to_string());
                }
                let row_data = data[..data.len() - 4].to_vec();
                *&mut data = &data[data.len() - 4..];
                let crc = read_u32(&mut data)?;
                JournalEntry::Delete {
                    timestamp,
                    table_id,
                    row_id,
                    row_data,
                    crc,
                }
            }
            _ => return Err(format!("Unknown entry type: {}", entry_type)),
        })
    }
}

#[cfg(test)]
#[path = "tests/t_journalentry.rs"]
mod t_journalentry;
