use strum::FromRepr;

/// Metadata for a field in a collection, supporting primitives and nested structs.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FieldMetadata {
    pub name: String,
    pub ty: Box<FieldType>,
}

/// Supported field types, including primitives and nested structs.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, FromRepr, Default)]
#[repr(u8)]
pub enum FieldType {
    /// 8-bit unsigned integer
    #[default]
    U8,
    /// 16-bit unsigned integer
    U16,
    /// 32-bit unsigned integer
    U32,
    /// 64-bit unsigned integer
    U64,
    /// 8-bit signed integer
    I8,
    /// 16-bit signed integer
    I16,
    /// 32-bit signed integer
    I32,
    /// 64-bit signed integer
    I64,
    /// 32-bit floating point
    F32,
    /// 64-bit floating point
    F64,
    /// Boolean
    Bool,
    /// String
    String,
    /// Struct
    Struct(FieldMetadata),
}

impl FieldType {
    /// Create a `FieldType` from a type name string.
    pub fn from_type_name(type_name: &str) -> Option<Self> {
        match type_name {
            "u8" => Some(FieldType::U8),
            "u16" => Some(FieldType::U16),
            "u32" => Some(FieldType::U32),
            "u64" => Some(FieldType::U64),
            "i8" => Some(FieldType::I8),
            "i16" => Some(FieldType::I16),
            "i32" => Some(FieldType::I32),
            "i64" => Some(FieldType::I64),
            "f32" => Some(FieldType::F32),
            "f64" => Some(FieldType::F64),
            "bool" => Some(FieldType::Bool),
            "String" | "&str" => Some(FieldType::String),
            _ => None,
        }
    }
}
