use core::ops::RangeInclusive;

/// Version number of the EpilogLite library
pub const EPILOGLITE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Magic header string for EPLite format 1
pub const EPLITE_MAGIC_HEADER_V1: &[u8; 16] = b"EPLite format 1\0";

/// Magic header string for SQLite format 3
pub const SQLITE_MAGIC_HEADER_V3: &[u8; 16] = b"SQLite format 3\0";

/// Valid page size range
pub const PAGE_SIZE_RANGE: RangeInclusive<u16> = 512..=65536;

/// Default page size (4096 bytes)
pub const DEFAULT_PAGE_SIZE: u32 = 4096;
