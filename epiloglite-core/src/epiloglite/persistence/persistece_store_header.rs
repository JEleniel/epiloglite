use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{SerializeError, SlotPointer, calculate_crc};

const EPILOGLITE_SIGNATURE: &str = "EpilogLite";
const EPILOGLITE_FILE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistenceStoreHeader {
    signature: String,               // "EpilogLite"
    version: u32,                    // EpilogLite file format version number
    page_size_exponent: u8, // Page size = (2^page_size_exponent)-1 bytes, where exponent is between 9 and 64; limited by usize::MAX - the maximum number of bytes addressable by a Vector
    page_count: u128,       // Number of pages in the store
    application_id: u128,   // Application identifier
    migration_version: u128, // Migration version of the store
    metadata_start: SlotPointer, // Pointer to the start of the metadata section
    index_master_start: SlotPointer, // Pointer to the start of the index master section
    index_free_start: SlotPointer, // Pointer to the start of the index free section
    crc: u32,               // CRC32 checksum of the header (excluding this field)
}

impl PersistenceStoreHeader {
    pub fn new(
        application_id: u128,
        migration_version: u128,
    ) -> Result<Self, PersistenceStoreHeaderError> {
        let mut header = Self {
            signature: EPILOGLITE_SIGNATURE.to_string(),
            version: EPILOGLITE_FILE_VERSION,
            page_size_exponent: 12, // Default to 4096 bytes
            page_count: 0,
            application_id,
            migration_version,
            crc: 0,
            metadata_start: SlotPointer::zero(),
            index_master_start: SlotPointer::zero(),
            index_free_start: SlotPointer::zero(),
        };
        header.crc = calculate_crc(&header)?;
        Ok(header)
    }

    pub fn set_page_size_exponent(
        &mut self,
        exponent: u8,
    ) -> Result<(), PersistenceStoreHeaderError> {
        self.page_size_exponent = exponent;
        self.crc = calculate_crc(self)?;
        Ok(())
    }

    pub fn page_count(&self) -> u128 {
        self.page_count
    }

    pub fn set_page_count(&mut self, page_count: u128) -> Result<(), PersistenceStoreHeaderError> {
        self.page_count = page_count;
        self.crc = calculate_crc(self)?;
        Ok(())
    }

    pub fn application_id(&self) -> u128 {
        self.application_id
    }
    pub fn migration_version(&self) -> u128 {
        self.migration_version
    }

    pub fn metadata_start(&self) -> &SlotPointer {
        &self.metadata_start
    }
    pub fn set_metadata_start(
        &mut self,
        pointer: SlotPointer,
    ) -> Result<(), PersistenceStoreHeaderError> {
        self.metadata_start = pointer;
        self.crc = calculate_crc(self)?;
        Ok(())
    }

    pub fn index_master_start(&self) -> &SlotPointer {
        &self.index_master_start
    }
    pub fn set_index_master_start(
        &mut self,
        pointer: SlotPointer,
    ) -> Result<(), PersistenceStoreHeaderError> {
        self.index_master_start = pointer;
        self.crc = calculate_crc(self)?;
        Ok(())
    }

    pub fn index_free_start(&self) -> &SlotPointer {
        &self.index_free_start
    }
    pub fn set_index_free_start(
        &mut self,
        pointer: SlotPointer,
    ) -> Result<(), PersistenceStoreHeaderError> {
        self.index_free_start = pointer;
        self.crc = calculate_crc(self)?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PersistenceStoreHeaderError {
    #[error("Invalid signature in store header")]
    InvalidSignature,
    #[error("Unsupported store version: {0}")]
    UnsupportedVersion(u32),
    #[error("CRC error: {0}")]
    CrcMismatch(#[from] SerializeError),
}
