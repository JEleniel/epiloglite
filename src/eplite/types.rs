pub mod column;
pub mod index;
pub mod orderby;
pub mod virtualtable;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Value {
    value: ValueType,
    changed: bool,
    bound: bool,
}

impl Value {
    pub fn new(value: ValueType) -> Self {
        Value {
            value,
            changed: false,
            bound: false,
        }
    }

    pub fn null() -> Self {
        Value::new(ValueType::Null)
    }

    pub fn value(&self) -> &ValueType {
        &self.value
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn bound(&self) -> bool {
        self.bound
    }

    pub fn mark_changed(&mut self) {
        self.changed = true;
    }

    pub fn mark_bound(&mut self) {
        self.bound = true;
    }
}

/// All possible value types in EpilogLite
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    /// NULL value
    Null,
    /// Boolean value
    Bool(bool),
    /// 8-bit signed integer
    I8(i8),
    /// 8-bit unsigned integer
    U8(u8),
    /// 16-bit signed integer
    I16(i16),
    /// 16-bit unsigned integer
    U16(u16),
    /// 32-bit signed integer
    I32(i32),
    /// 32-bit unsigned integer
    U32(u32),
    /// 64-bit signed integer
    I64(i64),
    /// 64-bit unsigned integer
    U64(u64),
    /// 128-bit signed integer
    I128(i128),
    /// 128-bit unsigned integer
    U128(u128),
    /// Pointer-sized signed integer
    ISize(isize),
    /// Pointer-sized unsigned integer
    USize(usize),
    /// 32-bit floating point
    F32(f32),
    /// 64-bit floating point (double)
    F64(f64),
    /// UTF-8 string
    StringUtf8(String),
    /// UTF-16 little endian string
    StringUtf16Le(Vec<u8>),
    /// UTF-16 big endian string
    StringUtf16Be(Vec<u8>),
    /// Binary blob
    Blob(Vec<u8>),
    /// Raw bytes
    Bytes(Vec<u8>),
}

impl ValueType {
    /// Get the type name as a string
    pub fn type_name(&self) -> &'static str {
        match self {
            ValueType::Null => "NULL",
            ValueType::Bool(_) => "BOOLEAN",
            ValueType::I8(_) => "INT8",
            ValueType::U8(_) => "UINT8",
            ValueType::I16(_) => "INT16",
            ValueType::U16(_) => "UINT16",
            ValueType::I32(_) => "INT32",
            ValueType::U32(_) => "UINT32",
            ValueType::I64(_) => "INT64",
            ValueType::U64(_) => "UINT64",
            ValueType::I128(_) => "INT128",
            ValueType::U128(_) => "UINT128",
            ValueType::ISize(_) => "INTSIZE",
            ValueType::USize(_) => "UINTSIZE",
            ValueType::F32(_) => "FLOAT32",
            ValueType::F64(_) => "FLOAT64",
            ValueType::StringUtf8(_) => "TEXT",
            ValueType::StringUtf16Le(_) => "TEXT_UTF16LE",
            ValueType::StringUtf16Be(_) => "TEXT_UTF16BE",
            ValueType::Blob(_) => "BLOB",
            ValueType::Bytes(_) => "BYTES",
        }
    }

    /// Check if the value is NULL
    pub fn is_null(&self) -> bool {
        matches!(self, ValueType::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_creation() {
        let val = Value::new(ValueType::I32(42));
        assert_eq!(val.value().type_name(), "INT32");
        assert!(!val.changed());
        assert!(!val.bound());
    }

    #[test]
    fn test_null_value() {
        let val = Value::null();
        assert!(val.value().is_null());
    }

    #[test]
    fn test_value_flags() {
        let mut val = Value::new(ValueType::Bool(true));
        assert!(!val.changed());
        val.mark_changed();
        assert!(val.changed());

        assert!(!val.bound());
        val.mark_bound();
        assert!(val.bound());
    }

    #[test]
    fn test_value_types() {
        assert_eq!(ValueType::Null.type_name(), "NULL");
        assert_eq!(ValueType::Bool(true).type_name(), "BOOLEAN");
        assert_eq!(ValueType::I32(42).type_name(), "INT32");
        assert_eq!(ValueType::F64(3.14).type_name(), "FLOAT64");
        assert_eq!(
            ValueType::StringUtf8("test".to_string()).type_name(),
            "TEXT"
        );
    }

    #[test]
    fn test_is_null() {
        assert!(ValueType::Null.is_null());
        assert!(!ValueType::I32(0).is_null());
    }
}
