use crate::RecordFlags;
/// Database file header parsing and serialization
use crate::{
    BINCODE_CONFIG, CIntError, CURRENT_FORMAT_VERSION, DEFAULT_PAGE_SIZE_EXPONENT,
    EPLITE_SIGNATURE, MAX_HEADER_SIZE, MIN_HEADER_SIZE, PAGE_SIZE_EXPONENT_RANGE, SerializeError,
    calculate_crc,
};
use crate::{Cu128, OffsetPointer};
use bincode::error::{DecodeError, EncodeError};
use flagset::{FlagSet, flags};
use serde::{Deserialize, Serialize};
use thiserror::Error;

flags! {
    /// Flags representing database-wide configuration or state.
    #[derive(Serialize, Deserialize)]
    pub enum StoreFlags: u8 {
        /// No flags set.
        None = 0,
        /// Database is opened read-only and write operations should fail.
        ReadOnly = 1 << 0,
        /// Authentication is required to access the database.
        AuthRequired = 1 << 1,
    }
}

/// Data Store header
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StoreHeader {
    /// Signature string (Magic Header)
    signature: String,
    /// File format
    format_version: u8,
    /// Page size exponent (Page size = 2^page_size_exponent bytes)
    /// Valid page size exponent range (9 to 64)
    /// Pages are indexed from 0 to (2^page_size_exponent)-1
    page_size_exponent: u8,
    /// Database flags
    flags: FlagSet<StoreFlags>,
    /// Location of the start of the Metadata list (Collection 1)
    metadata_pointer: OffsetPointer,
    /// Location of the start of the Free Page List (Collection 2)
    freelist_pointer: OffsetPointer,
    /// Location of the RowID Index (Collection 3)
    row_id_index_pointer: OffsetPointer,
    /// Application ID, for use by applications
    application_id: Cu128,
    /// Migration version, for use by applications
    migration_version: Cu128,
    /// Engine bookkeeping: record id for header (not persisted as part of the file layout)
    pub record_id: Option<Cu128>,
    /// Engine bookkeeping: transient record flags
    pub record_flags: flagset::FlagSet<RecordFlags>,
    /// Header CRC32 checksum
    crc: u32,
}

impl StoreHeader {
    /// Create a new DatabaseHeader with default values
    pub fn builder() -> Result<Self, HeaderError> {
        let mut header = Self {
            signature: EPLITE_SIGNATURE.to_string(),
            format_version: CURRENT_FORMAT_VERSION,
            page_size_exponent: DEFAULT_PAGE_SIZE_EXPONENT,
            flags: FlagSet::empty(),
            record_id: None,
            record_flags: FlagSet::empty(),
            freelist_pointer: OffsetPointer::null(),
            metadata_pointer: OffsetPointer::null(),
            row_id_index_pointer: OffsetPointer::null(),
            application_id: 0u128.into(),
            migration_version: 0u128.into(),
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
        if page_size_exponent < 9 {
            return Err(HeaderError::InvalidPageSizeExponent(page_size_exponent));
        }

        self.page_size_exponent = page_size_exponent;
        self.crc = calculate_crc(self)?;
        Ok(self)
    }

    /// Builder-style setter for flags
    pub fn with_flags(&mut self, flags: FlagSet<StoreFlags>) -> Result<&mut Self, HeaderError> {
        self.flags = flags;
        self.crc = calculate_crc(self)?;
        Ok(self)
    }

    /// Builder-style setter for application_id
    pub fn with_application_id(
        &mut self,
        application_id: &Cu128,
    ) -> Result<&mut Self, HeaderError> {
        self.application_id = application_id.clone();
        self.crc = calculate_crc(self)?;
        Ok(self)
    }

    /// Builder-style setter for migration_version
    pub fn with_migration_version(
        &mut self,
        migration_version: &Cu128,
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
        let crc = calculate_crc(self)?;
        if crc != self.crc {
            return Err(HeaderError::InvalidCRC(crc, self.crc));
        }
        Ok(true)
    }
}

impl TryFrom<&[u8]> for StoreHeader {
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
        let (header, _): (StoreHeader, _) =
            bincode::serde::decode_from_slice(value, BINCODE_CONFIG)?;
        Ok(header)
    }
}

impl TryInto<Vec<u8>> for &StoreHeader {
    type Error = EncodeError;

    /// Serialize the DatabaseHeader to a byte vector
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        bincode::serde::encode_to_vec(self, BINCODE_CONFIG)
    }
}

// Implement Record trait for StoreHeader so it can be used in Containers.
impl crate::Record for StoreHeader {
    fn record_id(&self) -> Option<u128> {
        self.record_id
            .clone()
            .and_then(|cu| u128::try_from(cu).ok())
    }

    fn set_record_id(&mut self, id: Option<u128>) {
        self.record_id = id.map(|v| Cu128::from(v));
    }

    fn flags(&self) -> &flagset::FlagSet<RecordFlags> {
        &self.record_flags
    }

    fn flags_mut(&mut self) -> &mut flagset::FlagSet<RecordFlags> {
        &mut self.record_flags
    }
}

/// Errors that can be returned while parsing a database header
#[derive(Debug, Error)]
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
    InvalidFreelistOffset(Cu128, Cu128, Cu128),
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
    /// Serialization error calculating CRC
    #[error("Serialization error calculating CRC {0:?}")]
    CrcCalculationError(#[from] SerializeError),
}
