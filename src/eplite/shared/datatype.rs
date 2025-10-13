//! EpilogLite column data types
use serde::{Deserialize, Serialize};
use strum::EnumString;

/// EpilogLite column data types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString)]
pub enum DataType {
    /// Rust `()`
    Null,
    /// Rust `boolean`
    Boolean,
    /// Rust `i8`
    I8,
    /// Rust `u8`
    U8,
    /// Rust `i16`
    I16,
    /// Rust `u16`
    U16,
    /// Rust `i32`
    I32,
    /// Rust `u32`
    U32,
    /// Rust `i64`
    I64,
    /// Rust `u64`
    U64,
    /// Rust `i128`
    I128,
    /// Rust `u128`
    U128,
    /// Rust `f32`
    F32,
    /// Rust `f64`
    F64,
    /// Rust String
    String(Option<u64>),
    /// Rust `Vec<u8>` or `&[u8]`
    ByteArray,
}

impl DataType {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_numeric() {
        use super::DataType;
        assert!(DataType::I32.is_numeric());
        assert!(DataType::F64.is_numeric());
        assert!(!DataType::String(None).is_numeric());
        assert!(!DataType::Null.is_numeric());
    }

    #[test]
    fn test_is_integer() {
        use super::DataType;
        assert!(DataType::I32.is_integer());
        assert!(!DataType::F64.is_integer());
        assert!(!DataType::String(None).is_integer());
        assert!(!DataType::Null.is_integer());
    }

    #[test]
    fn test_is_float() {
        use super::DataType;
        assert!(DataType::F64.is_float());
        assert!(!DataType::I32.is_float());
        assert!(!DataType::String(None).is_float());
        assert!(!DataType::Null.is_float());
    }
}
