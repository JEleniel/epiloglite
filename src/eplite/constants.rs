/// Database file format constants

/// Magic header string for EPLite format 1
pub const EPLITE_MAGIC_HEADER_V1: &[u8; 16] = b"EPLite format 1\0";

/// Magic header string for SQLite format 3
pub const SQLITE_MAGIC_HEADER_V3: &[u8; 16] = b"SQLite format 3\0";

/// Default page size (4096 bytes)
pub const DEFAULT_PAGE_SIZE: u32 = 4096;

/// Minimum page size (512 bytes)
pub const MIN_PAGE_SIZE: u32 = 512;

/// Maximum page size (65536 bytes)
pub const MAX_PAGE_SIZE: u32 = 65536;

/// Text encoding values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum TextEncoding {
	/// UTF-8 encoding (default)
	Utf8 = 1,
	/// UTF-16 little endian
	Utf16Le = 2,
	/// UTF-16 big endian
	Utf16Be = 3,
}

impl TextEncoding {
	pub fn from_u32(value: u32) -> Option<Self> {
		match value {
			1 => Some(TextEncoding::Utf8),
			2 => Some(TextEncoding::Utf16Le),
			3 => Some(TextEncoding::Utf16Be),
			_ => None,
		}
	}

	pub fn to_u32(self) -> u32 {
		self as u32
	}
}

/// Schema format numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl SchemaFormat {
	pub fn from_u32(value: u32) -> Option<Self> {
		match value {
			1 => Some(SchemaFormat::Original),
			2 => Some(SchemaFormat::AlterTableAddColumn),
			3 => Some(SchemaFormat::NonNullDefault),
			4 => Some(SchemaFormat::DescAndBoolean),
			_ => None,
		}
	}

	pub fn to_u32(self) -> u32 {
		self as u32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_magic_headers() {
		assert_eq!(EPLITE_MAGIC_HEADER_V1.len(), 16);
		assert_eq!(SQLITE_MAGIC_HEADER_V3.len(), 16);
	}

	#[test]
	fn test_page_sizes() {
		assert!(MIN_PAGE_SIZE < DEFAULT_PAGE_SIZE);
		assert!(DEFAULT_PAGE_SIZE < MAX_PAGE_SIZE);
		assert_eq!(MIN_PAGE_SIZE, 512);
		assert_eq!(DEFAULT_PAGE_SIZE, 4096);
		assert_eq!(MAX_PAGE_SIZE, 65536);
	}

	#[test]
	fn test_text_encoding() {
		assert_eq!(TextEncoding::Utf8.to_u32(), 1);
		assert_eq!(TextEncoding::from_u32(1), Some(TextEncoding::Utf8));
		assert_eq!(TextEncoding::from_u32(99), None);
	}

	#[test]
	fn test_schema_format() {
		assert_eq!(SchemaFormat::DescAndBoolean.to_u32(), 4);
		assert_eq!(
			SchemaFormat::from_u32(4),
			Some(SchemaFormat::DescAndBoolean)
		);
		assert_eq!(SchemaFormat::from_u32(99), None);
	}
}
