//! Configuration for EpilogLite

use crate::{
    constants::DEFAULT_PAGE_SIZE,
    persistence::{FileFormat, ReadWriteMode, TextEncoding, VacuumMode},
};

/// Database configuration
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseConfig {
    /// The format of the database file
    pub format: FileFormat,
    /// Page size in bytes (must be power of 2 between 512 and 65536)
    pub page_size: u32,
    /// Write mode
    pub write_mode: ReadWriteMode,
    /// Read mode
    pub read_mode: ReadWriteMode,
    /// Reserved space at end of each page
    pub reserved_bytes: u8,
    /// Page cache size in number of pages
    pub page_cache_size: i32,
    /// Text encoding
    pub text_encoding: TextEncoding,
    /// Which vacuum mode to use
    pub vacuum_mode: VacuumMode,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            page_size: DEFAULT_PAGE_SIZE,
            format: FileFormat::EpilogLite1,
            write_mode: ReadWriteMode::WriteAheadLog,
            read_mode: ReadWriteMode::WriteAheadLog,
            reserved_bytes: 0,
            page_cache_size: -2000,
            text_encoding: TextEncoding::Utf8,
            vacuum_mode: VacuumMode::EpilogLite,
        }
    }
}
