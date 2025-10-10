//! Database file header parsing and serialization

#[cfg(not(feature = "std"))]
use alloc::{string::ToString, vec, vec::Vec};
use thiserror::Error;

use crate::{EPILOGLITE_VERSION, EPLITE_MAGIC_HEADER_V1, SQLITE_MAGIC_HEADER_V3, semver::SemVer};

/// Database file header (first 100 bytes of the database file)
#[derive(Debug, Clone)]
pub struct DatabaseHeader {
    // Signature string (Magic Header)
    pub signature: String,
    /// File format
    pub format: FileFormat,
    /// Page size in bytes (must be power of 2 between 512 and 65536)
    pub page_size: u32,
    /// File format write version
    pub write_mode: ReadWriteMode,
    /// File format read version
    pub read_mode: ReadWriteMode,
    /// Reserved space at end of each page
    pub reserved_bytes: u8,
    /// Maximum embedded payload fraction (must be 64)
    pub max_embedded_payload: u8,
    /// Minimum embedded payload fraction (must be 32)
    pub min_embedded_payload: u8,
    /// Leaf payload fraction (must be 32)
    pub leaf_payload: u8,
    /// File change counter
    pub change_counter: u32,
    /// Size of database file in pages
    pub database_size: u32,
    /// Page number of first freelist trunk page
    pub first_freelist_page: u32,
    /// Total number of freelist pages
    pub freelist_pages: u32,
    /// Schema cookie
    pub schema_cookie: u32,
    /// Schema format number
    pub schema_format: SchemaFormat,
    /// Default page cache size
    pub page_cache_size: u32,
    /// Page number of largest root b-tree page
    pub largest_root_page: u32,
    /// Database text encoding
    pub text_encoding: TextEncoding,
    /// User version
    pub user_version: u32,
    /// Incremental vacuum mode
    pub incremental_vacuum: bool,
    /// Application ID
    pub application_id: u32,
    /// Reserved for expansion. Must be zero.
    pub reserved: [u8; 20],
    /// Version-valid-for number
    pub version_valid_for: u32,
    /// EpilogLite/SQLite version number
    pub xlite_version: u32,
}

impl DatabaseHeader {
    /// Create a new header with default values for EPLite format
    pub fn new_eplite() -> Self {
        DatabaseHeader {
            signature: String::from_utf8(EPLITE_MAGIC_HEADER_V1.to_vec()).unwrap(),
            format: FileFormat::EpilogLite1,
            page_size: 4096,
            write_mode: ReadWriteMode::WriteAheadLog,
            read_mode: ReadWriteMode::WriteAheadLog,
            reserved_bytes: 0,
            max_embedded_payload: 64,
            min_embedded_payload: 32,
            leaf_payload: 32,
            change_counter: 0,
            database_size: 0,
            first_freelist_page: 0,
            freelist_pages: 0,
            schema_cookie: 0,
            schema_format: SchemaFormat::EpilogLite,
            page_cache_size: 0,
            largest_root_page: 0,
            text_encoding: TextEncoding::Utf8,
            user_version: 0,
            incremental_vacuum: false,
            reserved: [0; 20],
            application_id: 0,
            version_valid_for: 0,
            xlite_version: SemVer::try_from(EPILOGLITE_VERSION).unwrap().into(),
        }
    }

    /// Parse a header from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HeaderError> {
        if bytes.len() != 100 {
            return Err(HeaderError::InvalidHeaderSize(bytes.len()));
        }

        // Check magic header
        let format = if &bytes[0..16] == EPLITE_MAGIC_HEADER_V1 {
            FileFormat::EpilogLite1
        } else if &bytes[0..16] == SQLITE_MAGIC_HEADER_V3 {
            FileFormat::SQLite3
        } else {
            return Err(HeaderError::InvalidHeaderSignature(
                String::from_utf8(bytes[0..16].to_vec()).unwrap(),
            ));
        };

        // Parse page size
        let page_size_raw = Self::u16_from_be_bytes(&bytes[16..=17]);
        let page_size = if page_size_raw == 1 {
            65536
        } else {
            page_size_raw as u32
        };

        // Parse write and read modes
        let write_mode = match ReadWriteMode::try_from(bytes[18]) {
            Ok(v) => v,
            Err(_) => return Err(HeaderError::InvalidReadWriteMode(bytes[18])),
        };
        // Parse write and read modes
        let read_mode = match ReadWriteMode::try_from(bytes[19]) {
            Ok(v) => v,
            Err(_) => return Err(HeaderError::InvalidReadWriteMode(bytes[19])),
        };

        // Parse text encoding
        let text_encoding_raw = Self::u32_from_be_bytes(&bytes[56..=59]);
        let text_encoding = match TextEncoding::try_from(text_encoding_raw) {
            Ok(v) => v,
            Err(_) => {
                return Err(HeaderError::InvalidTextEncoding(text_encoding_raw));
            }
        };

        // Parse schema format
        let schema_format_raw = Self::u32_from_be_bytes(&bytes[44..=47]);
        let schema_format = match SchemaFormat::try_from(schema_format_raw) {
            Ok(v) => v,
            Err(_) => {
                return Err(HeaderError::InvalidSchemaFormat(schema_format_raw));
            }
        };

        // Parse EpilogLite/SQLite version
        let version_raw = Self::u32_from_be_bytes(&bytes[96..=99]);
        let version = match SemVer::try_from(version_raw) {
            Ok(v) => v,
            Err(e) => {
                return Err(HeaderError::InvalidVersion(format!("{:?}", e)));
            }
        };

        Ok(DatabaseHeader {
            signature: match format {
                FileFormat::SQLite3 => String::from_utf8(SQLITE_MAGIC_HEADER_V3.to_vec()).unwrap(),
                FileFormat::EpilogLite1 => {
                    String::from_utf8(EPLITE_MAGIC_HEADER_V1.to_vec()).unwrap()
                }
            },
            format: format,
            page_size: page_size,
            write_mode: write_mode,
            read_mode: read_mode,
            reserved_bytes: bytes[20],
            max_embedded_payload: bytes[21],
            min_embedded_payload: bytes[22],
            leaf_payload: bytes[23],
            change_counter: Self::u32_from_be_bytes(&bytes[24..=27]),
            database_size: Self::u32_from_be_bytes(&bytes[28..=31]),
            first_freelist_page: Self::u32_from_be_bytes(&bytes[32..=35]),
            freelist_pages: Self::u32_from_be_bytes(&bytes[36..=39]),
            schema_cookie: Self::u32_from_be_bytes(&bytes[40..=43]),
            schema_format,
            page_cache_size: Self::u32_from_be_bytes(&bytes[48..=51]),
            largest_root_page: Self::u32_from_be_bytes(&bytes[52..=55]),
            text_encoding,
            user_version: Self::u32_from_be_bytes(&bytes[60..=63]),
            incremental_vacuum: Self::u32_from_be_bytes(&bytes[64..=67]) != 0,
            application_id: Self::u32_from_be_bytes(&bytes[68..=71]),
            version_valid_for: Self::u32_from_be_bytes(&bytes[92..=95]),
            xlite_version: version.into(),
            reserved: [0; 20],
        })
    }

    /// Serialize the header to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; 100];

        // Write magic header
        let magic = match self.format {
            FileFormat::EpilogLite1 => EPLITE_MAGIC_HEADER_V1,
            FileFormat::SQLite3 => SQLITE_MAGIC_HEADER_V3,
        };
        bytes[0..16].copy_from_slice(magic);

        // Write page size
        let page_size_raw = if self.page_size == 65536 {
            1u16
        } else {
            self.page_size as u16
        };
        bytes[16..18].copy_from_slice(&page_size_raw.to_be_bytes());

        // Write other fields
        bytes[18] = self.write_mode.into();
        bytes[19] = self.read_mode.into();
        bytes[20] = self.reserved_bytes;
        bytes[21] = self.max_embedded_payload;
        bytes[22] = self.min_embedded_payload;
        bytes[23] = self.leaf_payload;
        bytes[24..28].copy_from_slice(&self.change_counter.to_be_bytes());
        bytes[28..32].copy_from_slice(&self.database_size.to_be_bytes());
        bytes[32..36].copy_from_slice(&self.first_freelist_page.to_be_bytes());
        bytes[36..40].copy_from_slice(&self.freelist_pages.to_be_bytes());
        bytes[40..44].copy_from_slice(&self.schema_cookie.to_be_bytes());
        bytes[44..48].copy_from_slice(&(self.schema_format as u32).to_be_bytes());
        bytes[48..52].copy_from_slice(&self.page_cache_size.to_be_bytes());
        bytes[52..56].copy_from_slice(&self.largest_root_page.to_be_bytes());
        bytes[56..60].copy_from_slice(&(self.text_encoding as u32).to_be_bytes());
        bytes[60..64].copy_from_slice(&self.user_version.to_be_bytes());
        bytes[64..68].copy_from_slice(&(self.incremental_vacuum as u32).to_be_bytes());
        bytes[68..72].copy_from_slice(&self.application_id.to_be_bytes());
        bytes[92..96].copy_from_slice(&self.version_valid_for.to_be_bytes());
        bytes[96..100].copy_from_slice(&self.xlite_version.to_be_bytes());

        bytes
    }

    fn u16_from_be_bytes(bytes: &[u8]) -> u16 {
        u16::from_be_bytes([bytes[0], bytes[1]])
    }

    fn u32_from_be_bytes(bytes: &[u8]) -> u32 {
        u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
    }
}

/// Errors that can be returned while parsing a database header
#[derive(Error, Clone, Debug)]
pub enum HeaderError {
    #[error("Invalid header size {0}")]
    InvalidHeaderSize(usize),
    #[error("Invalid header signature (Magic String) {0}")]
    InvalidHeaderSignature(String),
    #[error("Invalid read or write mode {0}")]
    InvalidReadWriteMode(u8),
    #[error("Invalid text encoding {0}")]
    InvalidTextEncoding(u32),
    #[error("Invalid schema format {0}")]
    InvalidSchemaFormat(u32),
    #[error("Invalid version {0}")]
    InvalidVersion(String),
}

/// File formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileFormat {
    SQLite3,
    EpilogLite1,
}

/// Read and write modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadWriteMode {
    Legacy,
    WriteAheadLog,
}

impl TryFrom<u8> for ReadWriteMode {
    type Error = HeaderError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ReadWriteMode::Legacy),
            2 => Ok(ReadWriteMode::WriteAheadLog),
            _ => Err(HeaderError::InvalidReadWriteMode(value)),
        }
    }
}

impl Into<u8> for ReadWriteMode {
    fn into(self) -> u8 {
        self as u8
    }
}

/// Schema format numbers
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum SchemaFormat {
    /// Original SQLite format
    Original = 1,
    /// Added ALTER TABLE ... ADD COLUMN support
    AlterTableAddColumn = 2,
    /// Added non-NULL default support
    NonNullDefault = 3,
    /// Added DESC keyword and boolean support
    DescAndBoolean = 4,
    /// Original EpilogLite format (starting from 0x8000 to leave room for future SQLite formats)
    EpilogLite = 0x8000,
}

impl TryFrom<u32> for SchemaFormat {
    type Error = HeaderError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SchemaFormat::Original),
            2 => Ok(SchemaFormat::AlterTableAddColumn),
            3 => Ok(SchemaFormat::NonNullDefault),
            4 => Ok(SchemaFormat::DescAndBoolean),
            _ => Err(HeaderError::InvalidTextEncoding(value)),
        }
    }
}

impl Into<u32> for SchemaFormat {
    fn into(self) -> u32 {
        self as u32
    }
}

/// Text encoding formats
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum TextEncoding {
    /// UTF-8 encoding (default)
    Utf8 = 1,
    /// UTF-16 little endian
    Utf16Le = 2,
    /// UTF-16 big endian
    Utf16Be = 3,
}

impl TryFrom<u32> for TextEncoding {
    type Error = HeaderError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TextEncoding::Utf8),
            2 => Ok(TextEncoding::Utf16Le),
            3 => Ok(TextEncoding::Utf16Be),
            _ => Err(HeaderError::InvalidTextEncoding(value)),
        }
    }
}

impl Into<u32> for TextEncoding {
    fn into(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {}
