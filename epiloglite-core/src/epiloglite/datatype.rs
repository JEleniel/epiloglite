//! EpilogLite column data types
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
    /// Struct metadata: list of named fields and their datatypes
    Struct(Vec<Metadata>),
    /// Option wrapper around another DataType (represents `Option<T>`)
    Option(Box<DataType>),
    /// Tuple of other datatypes
    Tuple(Vec<DataType>),
    /// Rust `char`
    Char,
    /// Rust `isize`
    Isize,
    /// Rust `usize`
    Usize,
}

// Additional documentation for DataType enum:
// - Null: Represents a null value (unit type).
// - Boolean: Boolean type.
// - I8/U8/I16/U16/I32/U32/I64/U64/I128/U128: Signed/unsigned integer types.
// - F32/F64: Floating point types.
// - String(Option<u64>): String type, optionally with a maximum length.
// - ByteArray: Binary data.

impl DataType {
    /// Check if this is a numeric type
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float()
    }

    /// Check if this is a float type
    pub fn is_float(&self) -> bool {
        matches!(self, DataType::F32 | DataType::F64)
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
                | DataType::Isize
                | DataType::Usize
        )
    }
    // ...existing code (truncated for brevity)...
}

// Default impl removed: names now match Rust counterparts and EnumString is not used.

/// Metadata for a named field. Owned `String` is used so the metadata can be
/// serialized/deserialized and stored in the data store.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    pub name: String,
    pub dtype: DataType,
}

impl Metadata {
    pub fn new(name: impl Into<String>, dtype: DataType) -> Self {
        Metadata {
            name: name.into(),
            dtype,
        }
    }
}
