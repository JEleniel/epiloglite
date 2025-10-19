//! Database file header parsing and serialization
use crate::BINCODE_CONFIG;
use crate::CInt;
use crate::CIntError;
use crate::calculate_crc;
use crate::eplite::persistence::{
    CURRENT_FORMAT_VERSION, DEFAULT_PAGE_SIZE_EXPONENT, EPLITE_SIGNATURE, MAX_HEADER_SIZE,
    MIN_HEADER_SIZE, PAGE_SIZE_EXPONENT_RANGE,
};
use bincode::error::{DecodeError, EncodeError};
use epiloglite_core::OffsetPointer;
use epiloglite_core::{DatabaseFlags, SerializeError};
use flagset::FlagSet;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Database file header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatabaseHeader {
    /// Signature string (Magic Header)
    signature: String,
    /// File format
    format_version: u8,
    /// Page size exponent (Page size = 2^page_size_exponent bytes)
    /// Valid page size exponent range (9 to 63)
    page_size_exponent: u8,
    /// Flags
    flags: FlagSet<DatabaseFlags>,
    /// Location of the start of the Free Page List
    freelist_pointer: OffsetPointer,
    /// Location of the start of the Metadata list
    metadata_pointer: OffsetPointer,
    /// Location of the RowID Index
    rowidindex_pointer: OffsetPointer,
    /// Application ID, for use by applications
    application_id: CInt,
    /// Migration version, for use by applications
    migration_version: CInt,
    /// Header CRC32 checksum
    crc: u32,
}

impl DatabaseHeader {
    /// Create a new DatabaseHeader with default values
    pub fn builder() -> Result<Self, HeaderError> {
        let mut header = Self {
            signature: EPLITE_SIGNATURE.to_string(),
            format_version: CURRENT_FORMAT_VERSION,
            page_size_exponent: DEFAULT_PAGE_SIZE_EXPONENT,
            flags: FlagSet::empty(),
            freelist_pointer: OffsetPointer {
                page_id: CInt::zero(),
                offset: CInt::from(MAX_HEADER_SIZE as usize),
            },
            metadata_pointer: OffsetPointer {
                page_id: 1.into(),
                offset: CInt::from(MAX_HEADER_SIZE as usize),
            },
            rowidindex_pointer: OffsetPointer {
                page_id: 2.into(),
                offset: CInt::from(MAX_HEADER_SIZE as usize),
            },
            application_id: CInt::zero(),
            migration_version: CInt::zero(),
            crc: 0,
        };
        header.crc = calculate_crc(&header)?;
        Ok(header)
    }

    /// Builder-style setter for page_size_exponent
    pub fn with_page_size_exponent(
        &mut self,
        page_size_exponent: u8,
    ) -> Result<&mut Self, HeaderError> {
        self.page_size_exponent = page_size_exponent;
        self.crc = calculate_crc(self)?;
        Ok(self)
    }

    /// Builder-style setter for flags
    pub fn with_flags(&mut self, flags: FlagSet<DatabaseFlags>) -> Result<&mut Self, HeaderError> {
        self.flags = flags;
        self.crc = calculate_crc(self)?;
        Ok(self)
    }

    /// Builder-style setter for application_id
    pub fn with_application_id(&mut self, application_id: &CInt) -> Result<&mut Self, HeaderError> {
        self.application_id = application_id.clone();
        self.crc = calculate_crc(self)?;
        Ok(self)
    }

    /// Builder-style setter for migration_version
    pub fn with_migration_version(
        &mut self,
        migration_version: &CInt,
    ) -> Result<&mut Self, HeaderError> {
        self.migration_version = migration_version.clone();
        self.crc = calculate_crc(self)?;
        Ok(self)
    }

    /// The exponent used to calculate the page size (2^page_size_exponent)
    pub fn page_size_exponent(&self) -> u8 {
        self.page_size_exponent
    }

    /// Validate the header fields and CRC
    pub fn validate(&self) -> Result<bool, HeaderError> {
        if self.signature != EPLITE_SIGNATURE {
            return Err(HeaderError::InvalidHeaderSignature(self.signature.clone()));
        }
        if self.format_version > CURRENT_FORMAT_VERSION {
            return Err(HeaderError::FormatTooNew(self.format_version));
        }
        if !PAGE_SIZE_EXPONENT_RANGE.contains(&(self.page_size_exponent)) {
            return Err(HeaderError::InvalidPageSizeExponent(
                self.page_size_exponent(),
            ));
        }
        if self.freelist_pointer.page_id != CInt::zero()
            || self.freelist_pointer.offset != CInt::from(MAX_HEADER_SIZE as usize)
        {
            return Err(HeaderError::InvalidFreelistOffset(
                self.freelist_pointer.page_id.clone(),
                self.freelist_pointer.offset.clone(),
                CInt::from(MAX_HEADER_SIZE as usize),
            ));
        }
        let crc = calculate_crc(self)?;
        if crc != self.crc {
            return Err(HeaderError::InvalidCRC(crc, self.crc));
        }
        Ok(true)
    }
}

impl TryFrom<&[u8]> for DatabaseHeader {
    type Error = HeaderError;

    /// Deserialize a DatabaseHeader from a byte slice
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < MIN_HEADER_SIZE || value.len() > MAX_HEADER_SIZE {
            return Err(HeaderError::InvalidSize(
                value.len(),
                MIN_HEADER_SIZE,
                MAX_HEADER_SIZE,
            ));
        }
        let (header, _): (DatabaseHeader, _) =
            bincode::serde::decode_from_slice(value, BINCODE_CONFIG)?;
        Ok(header)
    }
}

impl TryInto<Vec<u8>> for &DatabaseHeader {
    type Error = EncodeError;

    /// Serialize the DatabaseHeader to a byte vector
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serde::encode_to_vec(self, BINCODE_CONFIG)
    }
}

/// Errors that can be returned while parsing a database header
#[derive(Error, Debug)]
pub enum HeaderError {
    /// Decoding error
    #[error("Decoding error {0:?}")]
    DecodingError(#[from] DecodeError),
    /// Encoding error
    #[error("Encoding error {0:?}")]
    EncodingError(#[from] EncodeError),
    /// Invalid header signature (Magic String)
    #[error("Invalid header signature (Magic String) {0}")]
    InvalidHeaderSignature(String),
    /// The format version is newer than this library supports
    #[error("The format is newer than this library supports {0}")]
    FormatTooNew(u8),
    /// Invalid freelist offset (must be page 0 and offset >= MAX_HEADER_SIZE)
    #[error("Invalid freelist offset ({0:?},{1:?}), expected(0,{2:?})")]
    InvalidFreelistOffset(CInt, CInt, CInt),
    /// Invalid page size (must be in range 2^9 to 2^128 bytes)
    #[error("Invalid page size exponent {0}, valid range is 9 to 63")]
    InvalidPageSizeExponent(u8),
    /// Invalid CRC32 checksum
    #[error("Invalid CRC, calculated {0:08X}, expected {1:08X}")]
    InvalidCRC(u32, u32),
    /// The byte array is the wrong size to be a header
    #[error("Byte array is the wrong size to be a header {0}, expected {1} to {2}")]
    InvalidSize(usize, usize, usize),
    /// Error decoding a CInt
    #[error("Error decoding CInt {0:?}")]
    InvalidCInt(#[from] CIntError),
    /// IO Error
    #[error("IO Error {0:?}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization error calculating CRC {0:?}")]
    CrcCalculationError(#[from] SerializeError),
}
