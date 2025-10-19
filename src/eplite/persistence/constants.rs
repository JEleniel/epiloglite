use std::ops::RangeInclusive;

/// Magic header string for EL database files
pub const EPLITE_SIGNATURE: &str = "EpilogLite";

/// Current file format version
pub const CURRENT_FORMAT_VERSION: u8 = 1;

/// Const default page size exponent (2^12=4KiB)
pub const DEFAULT_PAGE_SIZE_EXPONENT: u8 = 12;
/// Valid page size exponent range (9 to 64)
/// The page size in bytes is a usize = 2^page_size_exponent bytes
/// The number of pages is limited by usize, with a minimum of 4 and a maximum of 2^64-1 pages
/// The total database size has a minimum of 2KiB and a maximum of about 67.1QiB
pub const PAGE_SIZE_EXPONENT_RANGE: RangeInclusive<u8> = 9..=63;

/// Min header size in bytes
pub const MIN_HEADER_SIZE: usize = 22;
/// Max header size in bytes
pub const MAX_HEADER_SIZE: usize = 255;
/// Included space for future expansion

/// Free page guard values, front of page
pub const FREE_PAGE_FRONT_GUARD: u32 = 0xDECAFACE;
/// Free page guard values, back of page
pub const FREE_PAGE_BACK_GUARD: u32 = 0xECAFACED;

/// Fixed (reserved) container IDs
/// Container ID for free pages
pub const FREE_PAGE_CONTAINER_ID: u128 = 0;
/// Container ID for the freelist
pub const FREELIST_CONTAINER_ID: u128 = 1;
/// Container ID for the metadata
pub const METADATA_CONTAINER_ID: u128 = 2;
/// Container ID for the rowid index
pub const ROWIDINDEX_CONTAINER_ID: u128 = 3;
