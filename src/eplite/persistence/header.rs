/// Database file header parsing and serialization

use crate::eplite::constants::{
	SchemaFormat, TextEncoding, EPLITE_MAGIC_HEADER_V1, SQLITE_MAGIC_HEADER_V3,
};
use crate::eplite::error::{Error, Result};

/// Database file format type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
	/// EPLite format version 1
	EPLiteV1,
	/// SQLite format version 3
	SQLiteV3,
}

/// Database file header (first 100 bytes of the database file)
#[derive(Debug, Clone)]
pub struct DatabaseHeader {
	/// File format
	pub format: FileFormat,
	/// Page size in bytes (must be power of 2 between 512 and 65536)
	pub page_size: u32,
	/// File format write version
	pub write_version: u8,
	/// File format read version
	pub read_version: u8,
	/// Reserved space at end of each page
	pub reserved_space: u8,
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
	/// Version-valid-for number
	pub version_valid_for: u32,
	/// SQLite version number
	pub sqlite_version: u32,
}

impl DatabaseHeader {
	/// Create a new header with default values for EPLite format
	pub fn new_eplite() -> Self {
		DatabaseHeader {
			format: FileFormat::EPLiteV1,
			page_size: 4096,
			write_version: 1,
			read_version: 1,
			reserved_space: 0,
			max_embedded_payload: 64,
			min_embedded_payload: 32,
			leaf_payload: 32,
			change_counter: 0,
			database_size: 0,
			first_freelist_page: 0,
			freelist_pages: 0,
			schema_cookie: 0,
			schema_format: SchemaFormat::DescAndBoolean,
			page_cache_size: 0,
			largest_root_page: 0,
			text_encoding: TextEncoding::Utf8,
			user_version: 0,
			incremental_vacuum: false,
			application_id: 0,
			version_valid_for: 0,
			sqlite_version: 0,
		}
	}

	/// Create a new header with default values for SQLite format
	pub fn new_sqlite() -> Self {
		let mut header = Self::new_eplite();
		header.format = FileFormat::SQLiteV3;
		header
	}

	/// Parse a header from bytes
	pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
		if bytes.len() < 100 {
			return Err(Error::InvalidFormat(
				"Header must be at least 100 bytes".to_string(),
			));
		}

		// Check magic header
		let format = if &bytes[0..16] == EPLITE_MAGIC_HEADER_V1 {
			FileFormat::EPLiteV1
		} else if &bytes[0..16] == SQLITE_MAGIC_HEADER_V3 {
			FileFormat::SQLiteV3
		} else {
			return Err(Error::InvalidFormat("Invalid magic header".to_string()));
		};

		// Parse page size
		let page_size_raw = u16::from_be_bytes([bytes[16], bytes[17]]);
		let page_size = if page_size_raw == 1 {
			65536
		} else {
			page_size_raw as u32
		};

		// Parse text encoding
		let text_encoding_raw = u32::from_be_bytes([bytes[56], bytes[57], bytes[58], bytes[59]]);
		let text_encoding = TextEncoding::from_u32(text_encoding_raw)
			.ok_or_else(|| Error::InvalidFormat("Invalid text encoding".to_string()))?;

		// Parse schema format
		let schema_format_raw = u32::from_be_bytes([bytes[44], bytes[45], bytes[46], bytes[47]]);
		let schema_format = SchemaFormat::from_u32(schema_format_raw)
			.ok_or_else(|| Error::InvalidFormat("Invalid schema format".to_string()))?;

		Ok(DatabaseHeader {
			format,
			page_size,
			write_version: bytes[18],
			read_version: bytes[19],
			reserved_space: bytes[20],
			max_embedded_payload: bytes[21],
			min_embedded_payload: bytes[22],
			leaf_payload: bytes[23],
			change_counter: u32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
			database_size: u32::from_be_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
			first_freelist_page: u32::from_be_bytes([bytes[32], bytes[33], bytes[34], bytes[35]]),
			freelist_pages: u32::from_be_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]),
			schema_cookie: u32::from_be_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]),
			schema_format,
			page_cache_size: u32::from_be_bytes([bytes[48], bytes[49], bytes[50], bytes[51]]),
			largest_root_page: u32::from_be_bytes([bytes[52], bytes[53], bytes[54], bytes[55]]),
			text_encoding,
			user_version: u32::from_be_bytes([bytes[60], bytes[61], bytes[62], bytes[63]]),
			incremental_vacuum: u32::from_be_bytes([bytes[64], bytes[65], bytes[66], bytes[67]])
				!= 0,
			application_id: u32::from_be_bytes([bytes[68], bytes[69], bytes[70], bytes[71]]),
			version_valid_for: u32::from_be_bytes([bytes[92], bytes[93], bytes[94], bytes[95]]),
			sqlite_version: u32::from_be_bytes([bytes[96], bytes[97], bytes[98], bytes[99]]),
		})
	}

	/// Serialize the header to bytes
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = vec![0u8; 100];

		// Write magic header
		let magic = match self.format {
			FileFormat::EPLiteV1 => EPLITE_MAGIC_HEADER_V1,
			FileFormat::SQLiteV3 => SQLITE_MAGIC_HEADER_V3,
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
		bytes[18] = self.write_version;
		bytes[19] = self.read_version;
		bytes[20] = self.reserved_space;
		bytes[21] = self.max_embedded_payload;
		bytes[22] = self.min_embedded_payload;
		bytes[23] = self.leaf_payload;
		bytes[24..28].copy_from_slice(&self.change_counter.to_be_bytes());
		bytes[28..32].copy_from_slice(&self.database_size.to_be_bytes());
		bytes[32..36].copy_from_slice(&self.first_freelist_page.to_be_bytes());
		bytes[36..40].copy_from_slice(&self.freelist_pages.to_be_bytes());
		bytes[40..44].copy_from_slice(&self.schema_cookie.to_be_bytes());
		bytes[44..48].copy_from_slice(&self.schema_format.to_u32().to_be_bytes());
		bytes[48..52].copy_from_slice(&self.page_cache_size.to_be_bytes());
		bytes[52..56].copy_from_slice(&self.largest_root_page.to_be_bytes());
		bytes[56..60].copy_from_slice(&self.text_encoding.to_u32().to_be_bytes());
		bytes[60..64].copy_from_slice(&self.user_version.to_be_bytes());
		bytes[64..68].copy_from_slice(&(self.incremental_vacuum as u32).to_be_bytes());
		bytes[68..72].copy_from_slice(&self.application_id.to_be_bytes());
		bytes[92..96].copy_from_slice(&self.version_valid_for.to_be_bytes());
		bytes[96..100].copy_from_slice(&self.sqlite_version.to_be_bytes());

		bytes
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_eplite_header() {
		let header = DatabaseHeader::new_eplite();
		assert_eq!(header.format, FileFormat::EPLiteV1);
		assert_eq!(header.page_size, 4096);
		assert_eq!(header.text_encoding, TextEncoding::Utf8);
	}

	#[test]
	fn test_new_sqlite_header() {
		let header = DatabaseHeader::new_sqlite();
		assert_eq!(header.format, FileFormat::SQLiteV3);
		assert_eq!(header.page_size, 4096);
	}

	#[test]
	fn test_header_serialization() {
		let header = DatabaseHeader::new_eplite();
		let bytes = header.to_bytes();
		assert_eq!(bytes.len(), 100);
		assert_eq!(&bytes[0..16], EPLITE_MAGIC_HEADER_V1);
	}

	#[test]
	fn test_header_round_trip() {
		let header = DatabaseHeader::new_sqlite();
		let bytes = header.to_bytes();
		let parsed = DatabaseHeader::from_bytes(&bytes).unwrap();
		assert_eq!(parsed.format, header.format);
		assert_eq!(parsed.page_size, header.page_size);
		assert_eq!(parsed.text_encoding, header.text_encoding);
	}

	#[test]
	fn test_invalid_header() {
		let bytes = vec![0u8; 50];
		let result = DatabaseHeader::from_bytes(&bytes);
		assert!(result.is_err());
	}

	#[test]
	fn test_invalid_magic() {
		let mut bytes = vec![0u8; 100];
		bytes[0..16].copy_from_slice(b"INVALID_MAGIC!!!");
		let result = DatabaseHeader::from_bytes(&bytes);
		assert!(result.is_err());
	}
}
