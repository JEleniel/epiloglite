use std::ops::RangeInclusive;

/// Magic header string for EL database files
pub const EPLITE_SIGNATURE: &str = "EpilogLite";

/// Current file format version
pub const CURRENT_FORMAT_VERSION: u8 = 1;

/// Const default page size exponent (2^12=4KiB)
pub const DEFAULT_PAGE_SIZE_EXPONENT: u8 = 12;
/// Valid page size exponent range (9 to 64)
/// The page size in bytes = 2^page_size_exponent (2^9=512 bytes to 2^64=1.8E16 bytes)
/// The number of pages can be 3-2^64
/// The total database size has a minimum of 1536B and a maximum of about 3.3E35QiB
pub const PAGE_SIZE_RANGE: RangeInclusive<u8> = 9..=64;

/// Min header size in bytes
pub const MIN_HEADER_SIZE: usize = 21;
/// Max header size in bytes
pub const MAX_HEADER_SIZE: usize = 101;

/// Free page guard values, front of page
pub const FREE_PAGE_FRONT_GUARD: u32 = 0xDECAFACE;
/// Free page guard values, back of page
pub const FREE_PAGE_BACK_GUARD: u32 = 0xECAFACED;
