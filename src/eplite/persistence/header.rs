//! Database file header parsing and serialization
use std::{io::Read, ops::RangeInclusive};

use crc_adler::crc32;
use lowlevel_types::ascii;
use serde::{Deserialize, Serialize};
use serde_binary_adv::{self, Serializer, stream};
use thiserror::Error;

use crate::{CInt, persistence::offsetpointer::OffsetPointer};

/// Magic header string for EL database files
const EPLITE_SIGNATURE: &str = "EpilogLite";
/// Current file format version
const CURRENT_FORMAT_VERSION: u8 = 1;
/// Const default page size (4096 bytes)
pub const DEFAULT_PAGE_SIZE_EXPONENT: u8 = 12;
/// Valid page size range (2^9 to 2^128  bytes)
pub const PAGE_SIZE_RANGE: RangeInclusive<u128> = (2 ^ 9)..=(2 ^ 128);
/// Max header size in bytes
pub const MAX_HEADER_SIZE: u128 = 101;

/// Database file header
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatabaseHeader {
    // Signature string (Magic Header)
    pub signature: ascii::FixedLengthString<10>,
    /// File format
    pub format_version: u8,
    /// Page size exponent (Page size = 2^(page_size) bytes)
    page_size_exponent: u8,
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
            signature: ascii::FixedLengthString::try_from(EPLITE_SIGNATURE).unwrap(),
            format_version: CURRENT_FORMAT_VERSION,
            page_size_exponent: DEFAULT_PAGE_SIZE_EXPONENT,
            flags: CInt::from(0),
            freelist_offset: OffsetPointer {
                page_number: CInt::from(0),
                offset: CInt::from(0),
            },
            application_id: CInt::from(0),
            migration_version: CInt::from(0),
            crc: 0x00000000,
        };
        header.crc = header.calculate_crc();
        header
    }
}

impl DatabaseHeader {
    fn calculate_crc(&self) -> u32 {
        crc32(Serializer::to_bytes(self, true).unwrap().as_slice())
    }

    pub fn page_size(&self) -> u128 {
        1 << self.page_size_exponent
    }

    /// Read and validate a header from a reader
    pub fn try_from<T>(reader: &mut T) -> Result<Self, HeaderError>
    where
        T: Read,
    {
        let header: DatabaseHeader = stream::Deserializer::read_bytes(reader, true).unwrap();

        if !header.signature.eq(&EPLITE_SIGNATURE) {
            return Err(HeaderError::InvalidHeaderSignature(
                header.signature.to_string(),
            ));
        }

        if header.format_version > CURRENT_FORMAT_VERSION {
            return Err(HeaderError::FormatTooNew(header.format_version));
        }

        if !PAGE_SIZE_RANGE.contains(&(header.page_size())) {
            return Err(HeaderError::InvalidPageSize(header.page_size()));
        }

        if header.freelist_offset.page_number > CInt::from(0) {
            return Err(HeaderError::InvalidFreelistOffset(
                header.freelist_offset.page_number,
            ));
        }

        if header.freelist_offset.offset < CInt::from(MAX_HEADER_SIZE) {
            return Err(HeaderError::InvalidFreelistOffset(
                header.freelist_offset.offset,
            ));
        }

        let mut bytes = Serializer::to_bytes(&header, true).unwrap();
        let len = bytes.len();
        bytes[len - 5] = 0;
        bytes[len - 4] = 0;
        bytes[len - 3] = 0;
        bytes[len - 2] = 0;
        bytes[len - 1] = 0;
        let crc: u32 = crc32(&bytes);
        crc;

        if crc != header.crc {
            return Err(HeaderError::InvalidCRC(crc, header.crc));
        }
        Ok(header)
    }

    /// Serialize the header to a writer
    pub fn write_to<T>(&self, writer: &mut T) -> Result<(), std::io::Error>
    where
        T: std::io::Write,
    {
        stream::Serializer::write_bytes(writer, self, true).unwrap();
        Ok(())
    }
}

/// Errors that can be returned while parsing a database header
#[derive(Error, Clone, Debug)]
pub enum HeaderError {
    #[error("Invalid header signature (Magic String) {0}")]
    InvalidHeaderSignature(String),
    #[error("The format is newer than this library supports {0}")]
    FormatTooNew(u8),
    #[error("Invalid freelist offset {0:?}")]
    InvalidFreelistOffset(CInt),
    #[error("Invalid page size {0}, valid range is 2^9 to 2^128 bytes")]
    InvalidPageSize(u128),
    #[error("Invalid CRC, calculated {0:08X}, expected {1:08X}")]
    InvalidCRC(u32, u32),
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_default_header_is_valid() {
        let header = DatabaseHeader::default();
        assert_eq!(header.signature.to_string(), EPLITE_SIGNATURE);
        assert_eq!(header.format_version, CURRENT_FORMAT_VERSION);
        assert_eq!(header.page_size_exponent, DEFAULT_PAGE_SIZE_EXPONENT);
        assert_eq!(header.page_size(), 1 << DEFAULT_PAGE_SIZE_EXPONENT);
        assert_eq!(header.flags, CInt::from(0));
        assert_eq!(header.freelist_offset.page_number, CInt::from(0));
        assert_eq!(header.freelist_offset.offset, CInt::from(0));
        assert_eq!(header.application_id, CInt::from(0));
        assert_eq!(header.migration_version, CInt::from(0));
        assert_eq!(header.crc, header.calculate_crc());
    }

    #[test]
    fn test_header_serialization_and_deserialization() {
        let header = DatabaseHeader::default();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let parsed = DatabaseHeader::try_from(&mut cursor).unwrap();
        assert_eq!(header, parsed);
    }

    #[test]
    fn test_invalid_signature() {
        let mut header = DatabaseHeader::default();
        header.signature = ascii::FixedLengthString::try_from("WrongMagic").unwrap();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidHeaderSignature(s) => assert_eq!(s, "WrongMagic"),
            _ => panic!("Expected InvalidHeaderSignature"),
        }
    }

    #[test]
    fn test_format_too_new() {
        let mut header = DatabaseHeader::default();
        header.format_version = CURRENT_FORMAT_VERSION + 1;
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::FormatTooNew(v) => assert_eq!(v, CURRENT_FORMAT_VERSION + 1),
            _ => panic!("Expected FormatTooNew"),
        }
    }

    #[test]
    fn test_invalid_page_size() {
        let mut header = DatabaseHeader::default();
        header.page_size_exponent = 8; // 2^8 = 256, below valid range
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidPageSize(size) => assert_eq!(size, 1 << 8),
            _ => panic!("Expected InvalidPageSize"),
        }
    }

    #[test]
    fn test_invalid_freelist_offset_page_number() {
        let mut header = DatabaseHeader::default();
        header.freelist_offset.page_number = CInt::from(1);
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidFreelistOffset(n) => assert_eq!(n, CInt::from(1)),
            _ => panic!("Expected InvalidFreelistOffset"),
        }
    }

    #[test]
    fn test_invalid_freelist_offset_offset() {
        let mut header = DatabaseHeader::default();
        header.freelist_offset.offset = CInt::from(1); // less than MAX_HEADER_SIZE
        header.crc = header.calculate_crc();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidFreelistOffset(n) => assert_eq!(n, CInt::from(1)),
            _ => panic!("Expected InvalidFreelistOffset"),
        }
    }

    #[test]
    fn test_invalid_crc() {
        let mut header = DatabaseHeader::default();
        header.crc = 0xDEADBEEF; // wrong CRC
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let mut cursor = Cursor::new(buf);
        let err = DatabaseHeader::try_from(&mut cursor).unwrap_err();
        match err {
            HeaderError::InvalidCRC(calc, expected) => {
                assert_ne!(calc, expected);
                assert_eq!(expected, 0xDEADBEEF);
            }
            _ => panic!("Expected InvalidCRC"),
        }
    }
}
