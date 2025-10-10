#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};
use strum::EnumString;
use thiserror::Error;

/// EpilogLite column data types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString)]
pub enum DataType {
    /// Rust `()`, SQLite `NULL`
    Null,
    /// Rust `boolean`, SQLite `INTEGER`
    Boolean,
    /// Rust `i8`, SQLite `INTEGER`
    I8,
    /// Rust `u8`, SQLite `INTEGER`
    U8,
    /// Rust `i16`, SQLite `INTEGER`
    I16,
    /// Rust `u16`, SQLite `INTEGER`
    U16,
    /// Rust `i32`, SQLite `INTEGER`
    I32,
    /// Rust `u32`, SQLite `INTEGER`
    U32,
    /// Rust `i64`, SQLite `INTEGER`
    I64,
    /// Rust `u64`, SQLite `INTEGER`
    U64,
    /// Rust `i128`, SQLite `INTEGER`
    I128,
    /// Rust `u128`, SQLite `INTEGER`
    U128,
    /// Rust `f32`, SQLite `REAL`
    F32,
    /// Rust `f64`, SQLite `REAL`
    F64,
    /// Rust String, SQLite `TEXT`
    String,
    /// Rust `Vec<u8>` or `&[u8]`, SQLite `BLOB`
    ByteArray,
}

impl DataType {
    /// Get the SQL type name
    pub fn to_sql_type(&self) -> &'static str {
        match self {
            DataType::Null => "NULL",
            DataType::Boolean => "BOOLEAN",
            DataType::String => "TEXT",
            DataType::ByteArray | DataType::I128 | DataType::U128 => "BLOB",
            DataType::F32 | DataType::F64 => "REAL",
            DataType::I8
            | DataType::U8
            | DataType::I16
            | DataType::U16
            | DataType::I32
            | DataType::U32
            | DataType::I64
            | DataType::U64 => "INTEGER",
        }
    }

    /// Parse a SQL type name into a ColumnType
    pub fn try_from_sql_type(name: &str) -> Result<DataType, DataTypeError> {
        Ok(match name.to_uppercase().as_str() {
            "NULL" => DataType::Null,
            "INTEGER" => DataType::I64,
            "REAL" => DataType::F64,
            "TEXT" => DataType::String,
            "BLOB" => DataType::ByteArray,
            _ => {
                return Err(DataTypeError::InvalidSqlType(name.to_string()));
            }
        })
    }

    /// Check if this is a numeric type
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    /// Check if this is an integer type
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            DataType::I8
                | DataType::U8
                | DataType::I16
                | DataType::U16
                | DataType::I32
                | DataType::U32
                | DataType::I64
                | DataType::U64
                | DataType::I128
                | DataType::U128
        )
    }

    /// Check if this is a floating point type
    pub fn is_float(&self) -> bool {
        matches!(self, DataType::F32 | DataType::F64)
    }

    /// Check if this is a text type
    pub fn is_string(&self) -> bool {
        matches!(self, DataType::String)
    }

    /// Check if this is a blob type
    pub fn is_byte_array(&self) -> bool {
        matches!(self, DataType::ByteArray)
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DataTypeError {
    #[error("Invalid SQLite type {0}")]
    InvalidSqlType(String),
}

#[cfg(test)]
mod tests {}
