/// Column type definitions

use serde::{Deserialize, Serialize};

/// SQL column data types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColumnType {
	/// NULL type
	Null,
	/// Boolean
	Boolean,
	/// Signed 8-bit integer
	Int8,
	/// Unsigned 8-bit integer
	UInt8,
	/// Signed 16-bit integer
	Int16,
	/// Unsigned 16-bit integer
	UInt16,
	/// Signed 32-bit integer (INTEGER)
	Int32,
	/// Unsigned 32-bit integer
	UInt32,
	/// Signed 64-bit integer (BIGINT)
	Int64,
	/// Unsigned 64-bit integer
	UInt64,
	/// Signed 128-bit integer
	Int128,
	/// Unsigned 128-bit integer
	UInt128,
	/// 32-bit floating point (REAL)
	Float32,
	/// 64-bit floating point (DOUBLE)
	Float64,
	/// Variable length text (TEXT)
	Text,
	/// Variable length binary data (BLOB)
	Blob,
}

impl ColumnType {
	/// Get the SQL type name
	pub fn sql_name(&self) -> &'static str {
		match self {
			ColumnType::Null => "NULL",
			ColumnType::Boolean => "BOOLEAN",
			ColumnType::Int8 => "INT8",
			ColumnType::UInt8 => "UINT8",
			ColumnType::Int16 => "INT16",
			ColumnType::UInt16 => "UINT16",
			ColumnType::Int32 => "INTEGER",
			ColumnType::UInt32 => "UINT32",
			ColumnType::Int64 => "BIGINT",
			ColumnType::UInt64 => "UINT64",
			ColumnType::Int128 => "INT128",
			ColumnType::UInt128 => "UINT128",
			ColumnType::Float32 => "REAL",
			ColumnType::Float64 => "DOUBLE",
			ColumnType::Text => "TEXT",
			ColumnType::Blob => "BLOB",
		}
	}

	/// Parse a SQL type name into a ColumnType
	pub fn from_sql_name(name: &str) -> Option<Self> {
		match name.to_uppercase().as_str() {
			"NULL" => Some(ColumnType::Null),
			"BOOLEAN" | "BOOL" => Some(ColumnType::Boolean),
			"INT8" | "TINYINT" => Some(ColumnType::Int8),
			"UINT8" => Some(ColumnType::UInt8),
			"INT16" | "SMALLINT" => Some(ColumnType::Int16),
			"UINT16" => Some(ColumnType::UInt16),
			"INT32" | "INTEGER" | "INT" => Some(ColumnType::Int32),
			"UINT32" => Some(ColumnType::UInt32),
			"INT64" | "BIGINT" => Some(ColumnType::Int64),
			"UINT64" => Some(ColumnType::UInt64),
			"INT128" => Some(ColumnType::Int128),
			"UINT128" => Some(ColumnType::UInt128),
			"FLOAT32" | "REAL" | "FLOAT" => Some(ColumnType::Float32),
			"FLOAT64" | "DOUBLE" | "DOUBLE PRECISION" => Some(ColumnType::Float64),
			"TEXT" | "VARCHAR" | "CHAR" | "STRING" => Some(ColumnType::Text),
			"BLOB" | "BINARY" | "BYTES" => Some(ColumnType::Blob),
			_ => None,
		}
	}

	/// Check if this is a numeric type
	pub fn is_numeric(&self) -> bool {
		matches!(
			self,
			ColumnType::Int8
				| ColumnType::UInt8
				| ColumnType::Int16
				| ColumnType::UInt16
				| ColumnType::Int32
				| ColumnType::UInt32
				| ColumnType::Int64
				| ColumnType::UInt64
				| ColumnType::Int128
				| ColumnType::UInt128
				| ColumnType::Float32
				| ColumnType::Float64
		)
	}

	/// Check if this is an integer type
	pub fn is_integer(&self) -> bool {
		matches!(
			self,
			ColumnType::Int8
				| ColumnType::UInt8
				| ColumnType::Int16
				| ColumnType::UInt16
				| ColumnType::Int32
				| ColumnType::UInt32
				| ColumnType::Int64
				| ColumnType::UInt64
				| ColumnType::Int128
				| ColumnType::UInt128
		)
	}

	/// Check if this is a floating point type
	pub fn is_float(&self) -> bool {
		matches!(self, ColumnType::Float32 | ColumnType::Float64)
	}

	/// Check if this is a text type
	pub fn is_text(&self) -> bool {
		matches!(self, ColumnType::Text)
	}

	/// Check if this is a blob type
	pub fn is_blob(&self) -> bool {
		matches!(self, ColumnType::Blob)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sql_name() {
		assert_eq!(ColumnType::Int32.sql_name(), "INTEGER");
		assert_eq!(ColumnType::Text.sql_name(), "TEXT");
		assert_eq!(ColumnType::Float64.sql_name(), "DOUBLE");
	}

	#[test]
	fn test_from_sql_name() {
		assert_eq!(
			ColumnType::from_sql_name("INTEGER"),
			Some(ColumnType::Int32)
		);
		assert_eq!(
			ColumnType::from_sql_name("text"),
			Some(ColumnType::Text)
		);
		assert_eq!(
			ColumnType::from_sql_name("DOUBLE"),
			Some(ColumnType::Float64)
		);
		assert_eq!(ColumnType::from_sql_name("INVALID"), None);
	}

	#[test]
	fn test_type_checks() {
		assert!(ColumnType::Int32.is_numeric());
		assert!(ColumnType::Int32.is_integer());
		assert!(!ColumnType::Int32.is_float());

		assert!(ColumnType::Float64.is_numeric());
		assert!(ColumnType::Float64.is_float());
		assert!(!ColumnType::Float64.is_integer());

		assert!(ColumnType::Text.is_text());
		assert!(!ColumnType::Text.is_numeric());

		assert!(ColumnType::Blob.is_blob());
		assert!(!ColumnType::Blob.is_text());
	}

	#[test]
	fn test_boolean_type() {
		assert_eq!(ColumnType::Boolean.sql_name(), "BOOLEAN");
		assert_eq!(
			ColumnType::from_sql_name("BOOL"),
			Some(ColumnType::Boolean)
		);
	}
}
