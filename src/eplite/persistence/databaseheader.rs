//! Database file header parsing and serialization
use std::{
    io::{self, Read},
    ops::RangeInclusive,
};

use crc_adler::crc32;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{CInt, CIntError, persistence::offsetpointer::OffsetPointer};

/// Magic header string for EL database files
pub const EPLITE_SIGNATURE: &[u8] = b"EpilogLite";
/// Current file format version
pub const CURRENT_FORMAT_VERSION: u8 = 1;
/// Const default page size (4096 bytes)
pub const DEFAULT_PAGE_SIZE_EXPONENT: u8 = 12;
/// Valid page size range (2^9 to 2^128  bytes)
pub const PAGE_SIZE_RANGE: RangeInclusive<usize> = 0x200..=0x10000000000000000000000000000000;
/// Min header size in bytes
pub const MIN_HEADER_SIZE: usize = 21;
/// Max header size in bytes
pub const MAX_HEADER_SIZE: usize = 101;

/// Database file header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatabaseHeader {
    /// Signature string (Magic Header)
    pub signature: Vec<u8>,
    /// File format
    pub format_version: u8,
    /// Page size exponent (Page size = 2^(page_size) bytes)
    pub page_size_exponent: u8,
    /// Flags
    pub flags: CInt,
    /// Location of the start of the Free Page List
    pub freelist_offset: OffsetPointer,
    /// Application ID, for use by applications
    pub application_id: CInt,
    /// Migration version, for use by applications
    pub migration_version: CInt,
    /// Header CRC32 checksum
    pub crc: u32,
}

impl Default for DatabaseHeader {
    fn default() -> Self {
        let mut header = Self {
            signature: EPLITE_SIGNATURE.to_vec(),
            format_version: CURRENT_FORMAT_VERSION,
            page_size_exponent: DEFAULT_PAGE_SIZE_EXPONENT,
            flags: CInt::zero(),
            freelist_offset: OffsetPointer {
                page_number: CInt::zero(),
                offset: CInt::from(MAX_HEADER_SIZE as u128), // Always leave room for the header to change
            },
            application_id: CInt::zero(),
            migration_version: CInt::zero(),
            crc: 0x00000000,
        };
        header.crc = header.calculate_crc();
        header
    }
}

impl DatabaseHeader {
    /// Calculate the CRC32 checksum of the header (excluding the CRC field itself)
    pub fn calculate_crc(&self) -> u32 {
        let mut bytes = self.to_bytes();
        bytes.truncate(bytes.len() - 4);
        crc32(&bytes)
    }

    /// The calculated page size in bytes
    pub fn page_size(&self) -> usize {
        (1 as usize) << self.page_size_exponent
    }

    /// Get the bytes of the header
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.signature.clone();
        bytes.push(self.format_version);
        bytes.push(self.page_size_exponent);
        bytes.extend(&self.flags.bytes());
        bytes.extend(&self.freelist_offset.page_number.bytes());
        bytes.extend(&self.freelist_offset.offset.bytes());
        bytes.extend(&self.application_id.bytes());
        bytes.extend(&self.migration_version.bytes());
        bytes.extend(&self.crc.to_be_bytes());
        bytes
    }

    /// Parse a header from a byte slice
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, HeaderError> {
        if bytes.len() < MIN_HEADER_SIZE as usize || bytes.len() > MAX_HEADER_SIZE as usize {
            return Err(HeaderError::InvalidSize(
                bytes.len(),
                MIN_HEADER_SIZE,
                MAX_HEADER_SIZE,
            ));
        }
        let mut header = Self::default();
        header.signature = bytes[0..10].to_vec();
        header.format_version = bytes[10];
        header.page_size_exponent = bytes[11];
        let mut rem_bytes = bytes[12..].to_vec();

        header.flags = CInt::try_from(&mut rem_bytes)?;
        header.freelist_offset.page_number = CInt::try_from(&mut rem_bytes)?;
        header.freelist_offset.offset = CInt::try_from(&mut rem_bytes)?;
        header.application_id = CInt::try_from(&mut rem_bytes)?;
        header.migration_version = CInt::try_from(&mut rem_bytes)?;
        header.crc = u32::from_be_bytes([rem_bytes[0], rem_bytes[1], rem_bytes[2], rem_bytes[3]]);

        header.validate()?;

        Ok(header)
    }

    /// Read and validate a header from a reader
    pub fn try_from<T>(reader: &mut T) -> Result<Self, HeaderError>
    where
        T: Read,
    {
        let mut header = DatabaseHeader::default();

        let signature_buf = &mut [0u8; 10];
        reader.read_exact(signature_buf)?;
        header.signature = signature_buf.to_vec();

        let mut byte_buf = [0u8; 1];
        reader.read_exact(&mut byte_buf)?;
        header.format_version = byte_buf[0];

        reader.read_exact(&mut byte_buf)?;
        header.page_size_exponent = byte_buf[0];

        header.flags = CInt::read_from(reader)?;

        header.freelist_offset.page_number = CInt::read_from(reader)?;
        header.freelist_offset.offset = CInt::read_from(reader)?;

        header.application_id = CInt::read_from(reader)?;
        header.migration_version = CInt::read_from(reader)?;

        let mut crc_buf = [0u8; 4];
        reader.read_exact(&mut crc_buf)?;
        header.crc = u32::from_be_bytes(crc_buf);

        header.validate()?;

        Ok(header)
    }

    /// Serialize the header to a writer
    pub fn write_to<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        writer.write_all(&self.to_bytes())?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), HeaderError> {
        if self.signature != EPLITE_SIGNATURE {
            return Err(HeaderError::InvalidHeaderSignature(
                String::from_utf8_lossy(&self.signature).to_string(),
            ));
        }
        if self.format_version > CURRENT_FORMAT_VERSION {
            return Err(HeaderError::FormatTooNew(self.format_version));
        }
        if !PAGE_SIZE_RANGE.contains(&(self.page_size())) {
            return Err(HeaderError::InvalidPageSize(self.page_size()));
        }
        if self.freelist_offset.page_number != CInt::zero()
            || self.freelist_offset.offset != CInt::from(MAX_HEADER_SIZE as u128)
        {
            return Err(HeaderError::InvalidFreelistOffset(
                self.freelist_offset.page_number.clone(),
                self.freelist_offset.offset.clone(),
                CInt::from(MAX_HEADER_SIZE as u128),
            ));
        }
        let crc = self.calculate_crc();
        if crc != self.crc {
            return Err(HeaderError::InvalidCRC(crc, self.crc));
        }
        Ok(())
    }
}

/// Errors that can be returned while parsing a database header
#[derive(Error, Debug)]
pub enum HeaderError {
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
    #[error("Invalid page size {0}, valid range is 2^9 to 2^128 bytes")]
    InvalidPageSize(usize),
    /// Invalid CRC32 checksum
    #[error("Invalid CRC, calculated {0:08X}, expected {1:08X}")]
    InvalidCRC(u32, u32),
    /// The byte array is the wrong size to be a header
    #[error("Byte array is the wrong size to be a header {0}, expected {1} to {2}")]
    InvalidSize(usize, usize, usize),
    /// Error decoding a CInt
    #[error("Error decoding CInt {0:?}")]
    InvalidCInt(CIntError),
    /// IO Error
    #[error("IO Error {0:?}")]
    IoError(std::io::Error),
}

impl From<CIntError> for HeaderError {
    fn from(err: CIntError) -> Self {
        HeaderError::InvalidCInt(err)
    }
}

impl From<io::Error> for HeaderError {
    fn from(err: io::Error) -> Self {
        HeaderError::IoError(err)
    }
}

#[cfg(test)]
#[path = "tests/t_databaseheader.rs"]
mod t_databaseheader;
