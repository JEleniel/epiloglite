//! EpilogLite column data types
//!
//! This module defines the `DataType` enum used to describe column and
//! field types inside EpilogLite's in-memory structures and on-disk
//! metadata. It also provides a small `Metadata` struct for named fields
//! (used in `Struct` variants).
use std::rc::Rc;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq)]
/// Represents the Rust type of a column or a field in EpilogLite.
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
    /// Rust `isize`
    Isize,
    /// Rust `usize`
    Usize,
    /// Rust `f32`
    F32,
    /// Rust `f64`
    F64,
    /// Rust `char`
    Char,
    /// Rust String with optional maximum length (e.g., VARCHAR(N)).
    /// `None` means unspecified length.
    String(Option<usize>),
    /// User-defined enum type: list of variants with optional payload
    /// types per variant. Each entry is (variant_name, optional_payload).
    /// The payload, if present, is a `DataType` describing the shape of
    /// data carried by that variant (e.g., a `Tuple` or `Struct`).
    Enum(Vec<(String, Option<DataType>)>),
    /// Byte sequences are represented as arrays or vectors of `u8`
    /// using the `ArrayPrimitive(Box::new(DataType::U8), N)` or
    /// `VecPrimitive(Box::new(DataType::U8))` variants. Prefer
    /// `VecPrimitive(Box::new(DataType::U8))` for representing byte
    /// arrays and do not rely on any backwards-compatibility aliases.
    /// Struct metadata: list of named fields and their datatypes
    Struct(Vec<Metadata>),
    /// Option wrapper around another DataType (represents `Option<T>`)
    Option(Box<DataType>),
    /// Tuple of other datatypes
    Tuple(Vec<DataType>),
    /// A vector (Vec<T>) where T is a primitive datatype (e.g. i32, f64,
    /// char, bool). Vec<primitive> are serialized directly to binary.
    VecPrimitive(Box<DataType>),
    /// A vector (Vec<T>) where T is a complex datatype (e.g. Struct,
    /// Tuple, Option, or nested vectors). Vec<complex> are converted to
    /// separate Containers with relationship pointers.
    VecComplex(Box<DataType>),
    /// A fixed-size array `[T; N]` where `T` is a primitive datatype.
    /// The `usize` element carries the compile-time array length `N`.
    ArrayPrimitive(Box<DataType>, usize),
    /// A fixed-size array `[T; N]` where `T` is a complex datatype.
    /// Stored separately from primitive arrays so callers can choose
    /// different storage or serialization strategies.
    ArrayComplex(Box<DataType>, usize),
}

/// Trait to infer a `DataType` from a Rust type `T` at compile time.
pub trait InferDataType {
    /// Return the `DataType` corresponding to `Self`.
    fn infer() -> DataType;
}

// Primitive types
macro_rules! impl_infer_prim {
    ($ty:ty, $variant:ident) => {
        impl InferDataType for $ty {
            fn infer() -> DataType {
                DataType::$variant
            }
        }
    };
}

impl_infer_prim!(bool, Boolean);
impl_infer_prim!(i8, I8);
impl_infer_prim!(u8, U8);
impl_infer_prim!(i16, I16);
impl_infer_prim!(u16, U16);
impl_infer_prim!(i32, I32);
impl_infer_prim!(u32, U32);
impl_infer_prim!(i64, I64);
impl_infer_prim!(u64, U64);
impl_infer_prim!(i128, I128);
impl_infer_prim!(u128, U128);
impl_infer_prim!(isize, Isize);
impl_infer_prim!(usize, Usize);
impl_infer_prim!(f32, F32);
impl_infer_prim!(f64, F64);
impl_infer_prim!(char, Char);

impl InferDataType for () {
    fn infer() -> DataType {
        DataType::Null
    }
}

impl InferDataType for String {
    fn infer() -> DataType {
        DataType::String(None)
    }
}

impl<'a> InferDataType for &'a str {
    fn infer() -> DataType {
        DataType::String(None)
    }
}

// Pointer/wrapper normalization: forward inference to the inner type.
impl<T: InferDataType> InferDataType for Box<T> {
    fn infer() -> DataType {
        T::infer()
    }
}

impl<T: InferDataType> InferDataType for Rc<T> {
    fn infer() -> DataType {
        T::infer()
    }
}

impl<T: InferDataType> InferDataType for Arc<T> {
    fn infer() -> DataType {
        T::infer()
    }
}

impl<T: InferDataType> InferDataType for &T {
    fn infer() -> DataType {
        T::infer()
    }
}

impl<T: InferDataType> InferDataType for &mut T {
    fn infer() -> DataType {
        T::infer()
    }
}

// Slices infer as vectors of the element type.
impl<'a, T: InferDataType> InferDataType for &'a [T] {
    fn infer() -> DataType {
        let inner = T::infer();
        if inner.is_primitive() {
            DataType::VecPrimitive(Box::new(inner))
        } else {
            DataType::VecComplex(Box::new(inner))
        }
    }
}

// Vec<T> inference
impl<T: InferDataType> InferDataType for Vec<T> {
    fn infer() -> DataType {
        let inner = T::infer();
        if inner.is_primitive() {
            DataType::VecPrimitive(Box::new(inner))
        } else {
            DataType::VecComplex(Box::new(inner))
        }
    }
}

// Option<T>
impl<T: InferDataType> InferDataType for Option<T> {
    fn infer() -> DataType {
        DataType::Option(Box::new(T::infer()))
    }
}

// Fixed-size arrays via const generics
impl<T: InferDataType, const N: usize> InferDataType for [T; N] {
    fn infer() -> DataType {
        let inner = T::infer();
        if inner.is_primitive() {
            DataType::ArrayPrimitive(Box::new(inner), N)
        } else {
            DataType::ArrayComplex(Box::new(inner), N)
        }
    }
}

// Tuple implementations for InferDataType generated up to arity 12.
macro_rules! impl_infer_tuple {
        ($($name:ident)+) => {
            impl<$($name: InferDataType),+> InferDataType for ($($name,)+) {
                fn infer() -> DataType {
                    let mut v = Vec::new();
                    $(v.push($name::infer());)+
                    DataType::Tuple(v)
                }
            }
        };
    }

impl_infer_tuple! { T1 }
impl_infer_tuple! { T1 T2 }
impl_infer_tuple! { T1 T2 T3 }
impl_infer_tuple! { T1 T2 T3 T4 }
impl_infer_tuple! { T1 T2 T3 T4 T5 }
impl_infer_tuple! { T1 T2 T3 T4 T5 T6 }
impl_infer_tuple! { T1 T2 T3 T4 T5 T6 T7 }
impl_infer_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 }
impl_infer_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 }
impl_infer_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 }
impl_infer_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 }
impl_infer_tuple! { T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 }

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

    /// Returns true when this data type is a vector of primitive types.
    pub fn is_vec_primitive(&self) -> bool {
        matches!(self, DataType::VecPrimitive(_))
    }

    /// Returns true when this data type is a vector of complex types.
    pub fn is_vec_complex(&self) -> bool {
        matches!(self, DataType::VecComplex(_))
    }

    /// Return true when the data type is considered a primitive scalar
    /// according to Rust's classification (integers, floats, booleans,
    /// characters, and the unit type `()`). `String` is explicitly not a
    /// scalar here and is treated as a compound type instead.
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            DataType::Null
                | DataType::Boolean
                | DataType::I8
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
                | DataType::F32
                | DataType::F64
                | DataType::Char
        )
    }

    /// Alias for `is_primitive` — a scalar is a single, non-structured
    /// value in Rust (integers, floats, bool, char, unit).
    pub fn is_scalar(&self) -> bool {
        self.is_primitive()
    }

    /// Return true when the type is complex/structured (Struct,
    /// Option, or container types that are stored/represented as
    /// separate records/structures). Tuples are considered compound
    /// (see `is_compound`) rather than complex.
    pub fn is_complex(&self) -> bool {
        matches!(
            self,
            DataType::Struct(_)
                | DataType::Option(_)
                | DataType::VecComplex(_)
                | DataType::ArrayComplex(_, _)
        )
    }

    /// Return true when the type is a compound type per Rust's terminology
    /// (tuple or array) or commonly treated as compound here (e.g.,
    /// `String`). Compound types group multiple values into a single
    /// entity.
    pub fn is_compound(&self) -> bool {
        matches!(
            self,
            DataType::Tuple(_)
                | DataType::ArrayPrimitive(_, _)
                | DataType::ArrayComplex(_, _)
                | DataType::String(_)
        )
    }

    /// Return true when this type is a fixed-size array `[T; N]`.
    pub fn is_array(&self) -> bool {
        matches!(
            self,
            DataType::ArrayPrimitive(_, _) | DataType::ArrayComplex(_, _)
        )
    }

    /// Return true when this type is a dynamic vector `Vec<T>`.
    pub fn is_vec(&self) -> bool {
        matches!(self, DataType::VecPrimitive(_) | DataType::VecComplex(_))
    }

    /// Returns true when this data type is a fixed-size array of primitive
    /// elements (e.g. `[i32; 4]`).
    pub fn is_array_primitive(&self) -> bool {
        matches!(self, DataType::ArrayPrimitive(_, _))
    }

    /// Returns true when this data type is a fixed-size array of complex
    /// elements (e.g. `[Struct; 3]`).
    pub fn is_array_complex(&self) -> bool {
        matches!(self, DataType::ArrayComplex(_, _))
    }

    /// Checked constructor for Enum metadata.
    pub fn enum_checked(variants: Vec<String>) -> Result<Self, DataTypeError> {
        if variants.is_empty() {
            Err(DataTypeError::ExpectedComplex("empty enum".to_string()))
        } else {
            // Convert simple Vec<String> into Vec<(String, Option<DataType>)>
            let v = variants
                .into_iter()
                .map(|s| (s, Option::<DataType>::None))
                .collect();
            Ok(DataType::Enum(v))
        }
    }

    /// Infer the `DataType` for a Rust type `T` that implements
    /// `InferDataType`.
    pub fn of<T: InferDataType>() -> DataType {
        T::infer()
    }

    /// Infer the `DataType` from a value. This simply forwards to the
    /// `InferDataType` implementation for `T` and exists for ergonomics.
    pub fn of_val<T: InferDataType>(_v: &T) -> DataType {
        T::infer()
    }

    // ---------- Strict constructors and validation ----------

    /// Create a `VecPrimitive` if `inner` is a primitive scalar type.
    pub fn vec_primitive_checked(inner: DataType) -> Result<Self, DataTypeError> {
        if inner.is_primitive() {
            Ok(DataType::VecPrimitive(Box::new(inner)))
        } else {
            Err(DataTypeError::ExpectedPrimitive(format!("{:?}", inner)))
        }
    }

    /// Create a `VecComplex` if `inner` is NOT a primitive scalar.
    pub fn vec_complex_checked(inner: DataType) -> Result<Self, DataTypeError> {
        if !inner.is_primitive() {
            Ok(DataType::VecComplex(Box::new(inner)))
        } else {
            Err(DataTypeError::ExpectedComplex(format!("{:?}", inner)))
        }
    }

    /// Create an `ArrayPrimitive` for a `[T; len]` when `inner` is
    /// primitive and `len > 0`.
    pub fn array_primitive_checked(inner: DataType, len: usize) -> Result<Self, DataTypeError> {
        if len == 0 {
            return Err(DataTypeError::InvalidArrayLength(len));
        }
        if inner.is_primitive() {
            Ok(DataType::ArrayPrimitive(Box::new(inner), len))
        } else {
            Err(DataTypeError::ExpectedPrimitive(format!("{:?}", inner)))
        }
    }

    /// Create an `ArrayComplex` for a `[T; len]` when `inner` is complex
    /// (not primitive) and `len > 0`.
    pub fn array_complex_checked(inner: DataType, len: usize) -> Result<Self, DataTypeError> {
        if len == 0 {
            return Err(DataTypeError::InvalidArrayLength(len));
        }
        if !inner.is_primitive() {
            Ok(DataType::ArrayComplex(Box::new(inner), len))
        } else {
            Err(DataTypeError::ExpectedComplex(format!("{:?}", inner)))
        }
    }

    /// Validate the `DataType` invariants recursively.
    ///
    /// Ensures that `VecPrimitive`/`ArrayPrimitive` actually contain
    /// primitive inners and that array lengths are positive.
    pub fn validate(&self) -> Result<(), DataTypeError> {
        match self {
            DataType::VecPrimitive(inner) => {
                if inner.is_primitive() {
                    Ok(())
                } else {
                    Err(DataTypeError::ExpectedPrimitive(format!("{:?}", inner)))
                }
            }
            DataType::VecComplex(inner) => {
                if !inner.is_primitive() {
                    Ok(())
                } else {
                    Err(DataTypeError::ExpectedComplex(format!("{:?}", inner)))
                }
            }
            DataType::ArrayPrimitive(inner, len) => {
                if *len == 0 {
                    return Err(DataTypeError::InvalidArrayLength(*len));
                }
                if inner.is_primitive() {
                    Ok(())
                } else {
                    Err(DataTypeError::ExpectedPrimitive(format!("{:?}", inner)))
                }
            }
            DataType::ArrayComplex(inner, len) => {
                if *len == 0 {
                    return Err(DataTypeError::InvalidArrayLength(*len));
                }
                if !inner.is_primitive() {
                    Ok(())
                } else {
                    Err(DataTypeError::ExpectedComplex(format!("{:?}", inner)))
                }
            }
            DataType::Tuple(items) => {
                for it in items {
                    it.validate()?;
                }
                Ok(())
            }
            DataType::Option(inner) => inner.validate(),
            DataType::Struct(fields) => {
                for m in fields {
                    m.dtype.validate()?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

/// Metadata for a named field.
///
/// `Metadata` holds the field name and its `DataType`. The `String` is
/// owned so the metadata may be serialized and stored in the engine's
/// persistence layers.
#[derive(Clone, Debug, PartialEq)]
pub struct Metadata {
    /// Field name
    pub name: String,

    /// Field datatype
    pub dtype: DataType,
}

impl Metadata {
    /// Create new `Metadata` for a field with `name` and `dtype`.
    pub fn new(name: impl Into<String>, dtype: DataType) -> Self {
        Metadata {
            name: name.into(),
            dtype,
        }
    }
}

/// Errors produced when constructing or validating `DataType` values.
#[derive(Debug, Error)]
pub enum DataTypeError {
    /// Returned when a primitive type was expected but a non-primitive
    /// was provided.
    #[error("expected primitive type, got: {0}")]
    ExpectedPrimitive(String),

    /// Returned when a complex type was expected but a primitive was
    /// provided.
    #[error("expected complex type, got: {0}")]
    ExpectedComplex(String),

    /// Returned when an array length is invalid (e.g. zero).
    #[error("invalid array length: {0}")]
    InvalidArrayLength(usize),
}

// Custom serde support: we implement Serialize/Deserialize manually to
// provide a stable, compact binary format (tag + payload) while keeping
// human-readable formats (JSON) friendly.
impl serde::Serialize for DataType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            // Use externally tagged enum style for readability
            match self {
                DataType::Null => serializer.serialize_str("Null"),
                DataType::Boolean => serializer.serialize_str("Boolean"),
                DataType::I8 => serializer.serialize_str("I8"),
                DataType::U8 => serializer.serialize_str("U8"),
                DataType::I16 => serializer.serialize_str("I16"),
                DataType::U16 => serializer.serialize_str("U16"),
                DataType::I32 => serializer.serialize_str("I32"),
                DataType::U32 => serializer.serialize_str("U32"),
                DataType::I64 => serializer.serialize_str("I64"),
                DataType::U64 => serializer.serialize_str("U64"),
                DataType::I128 => serializer.serialize_str("I128"),
                DataType::U128 => serializer.serialize_str("U128"),
                DataType::Isize => serializer.serialize_str("Isize"),
                DataType::Usize => serializer.serialize_str("Usize"),
                DataType::F32 => serializer.serialize_str("F32"),
                DataType::F64 => serializer.serialize_str("F64"),
                DataType::Char => serializer.serialize_str("Char"),
                DataType::String(opt) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("String", opt)?;
                    m.end()
                }
                DataType::Enum(vars) => {
                    use serde::ser::SerializeMap;
                    // Serialize enum as list of { name: <string>, payload: <datatype or null> }
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("Enum", vars)?;
                    m.end()
                }
                DataType::Struct(fields) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("Struct", fields)?;
                    m.end()
                }
                DataType::Option(inner) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("Option", inner)?;
                    m.end()
                }
                DataType::Tuple(items) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("Tuple", items)?;
                    m.end()
                }
                DataType::VecPrimitive(inner) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("VecPrimitive", inner)?;
                    m.end()
                }
                // Byte arrays are represented as VecPrimitive(U8)
                DataType::VecComplex(inner) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("VecComplex", inner)?;
                    m.end()
                }
                DataType::ArrayPrimitive(inner, len) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("ArrayPrimitive", &(inner, len))?;
                    m.end()
                }
                DataType::ArrayComplex(inner, len) => {
                    use serde::ser::SerializeMap;
                    let mut m = serializer.serialize_map(Some(1))?;
                    m.serialize_entry("ArrayComplex", &(inner, len))?;
                    m.end()
                }
            }
        } else {
            // Compact binary representation: (version: u8, tag: u8, payload)
            // Version byte allows future format changes.
            use serde::ser::SerializeTuple;
            let mut t = serializer.serialize_tuple(3)?;
            let version: u8 = 1u8;
            let tag: u8 = match self {
                DataType::Null => 0,
                DataType::Boolean => 1,
                DataType::I8 => 2,
                DataType::U8 => 3,
                DataType::I16 => 4,
                DataType::U16 => 5,
                DataType::I32 => 6,
                DataType::U32 => 7,
                DataType::I64 => 8,
                DataType::U64 => 9,
                DataType::I128 => 10,
                DataType::U128 => 11,
                DataType::Isize => 12,
                DataType::Usize => 13,
                DataType::F32 => 14,
                DataType::F64 => 15,
                DataType::Char => 16,
                DataType::String(_) => 17,
                DataType::Enum(_) => 18,
                DataType::Struct(_) => 19,
                DataType::Option(_) => 20,
                DataType::Tuple(_) => 21,
                DataType::VecPrimitive(_) => 22,

                DataType::VecComplex(_) => 23,
                DataType::ArrayPrimitive(_, _) => 24,
                DataType::ArrayComplex(_, _) => 25,
            };
            t.serialize_element(&version)?;
            t.serialize_element(&tag)?;
            // Payload follows; for unit-like variants we serialize an empty
            // sequence as payload.
            match self {
                DataType::String(opt) => t.serialize_element(opt)?,
                DataType::Enum(vars) => t.serialize_element(vars)?,
                DataType::Struct(fields) => t.serialize_element(fields)?,
                DataType::Option(inner) => t.serialize_element(inner)?,
                DataType::Tuple(items) => t.serialize_element(items)?,
                DataType::VecPrimitive(inner) => t.serialize_element(inner)?,

                DataType::VecComplex(inner) => t.serialize_element(inner)?,
                DataType::ArrayPrimitive(inner, len) => t.serialize_element(&(&**inner, len))?,
                DataType::ArrayComplex(inner, len) => t.serialize_element(&(&**inner, len))?,
                _ => t.serialize_element(&())?,
            }
            t.end()
        }
    }
}

impl<'de> serde::Deserialize<'de> for DataType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if deserializer.is_human_readable() {
            // Accept string tags or single-key maps.
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = DataType;

                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "a DataType representation")
                }

                fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                    match v {
                        "Null" => Ok(DataType::Null),
                        "Boolean" => Ok(DataType::Boolean),
                        "I8" => Ok(DataType::I8),
                        "U8" => Ok(DataType::U8),
                        "I16" => Ok(DataType::I16),
                        "U16" => Ok(DataType::U16),
                        "I32" => Ok(DataType::I32),
                        "U32" => Ok(DataType::U32),
                        "I64" => Ok(DataType::I64),
                        "U64" => Ok(DataType::U64),
                        "I128" => Ok(DataType::I128),
                        "U128" => Ok(DataType::U128),
                        "Isize" => Ok(DataType::Isize),
                        "Usize" => Ok(DataType::Usize),
                        "F32" => Ok(DataType::F32),
                        "F64" => Ok(DataType::F64),
                        "Char" => Ok(DataType::Char),
                        other => Err(serde::de::Error::unknown_variant(other, &[])),
                    }
                }

                fn visit_map<M: serde::de::MapAccess<'de>>(
                    self,
                    mut map: M,
                ) -> Result<Self::Value, M::Error> {
                    if let Some(key) = map.next_key::<String>()? {
                        match key.as_str() {
                            "String" => {
                                let opt: Option<usize> = map.next_value()?;
                                Ok(DataType::String(opt))
                            }
                            "Enum" => {
                                let vars: Vec<(String, Option<DataType>)> = map.next_value()?;
                                Ok(DataType::Enum(vars))
                            }
                            "Struct" => {
                                let fields: Vec<Metadata> = map.next_value()?;
                                Ok(DataType::Struct(fields))
                            }
                            "Option" => {
                                let inner: DataType = map.next_value()?;
                                Ok(DataType::Option(Box::new(inner)))
                            }
                            "Tuple" => {
                                let items: Vec<DataType> = map.next_value()?;
                                Ok(DataType::Tuple(items))
                            }
                            "VecPrimitive" => {
                                let inner: DataType = map.next_value()?;
                                Ok(DataType::VecPrimitive(Box::new(inner)))
                            }
                            // "ByteArray" short form removed; use VecPrimitive(U8)
                            "VecComplex" => {
                                let inner: DataType = map.next_value()?;
                                Ok(DataType::VecComplex(Box::new(inner)))
                            }
                            "ArrayPrimitive" => {
                                let pair: (DataType, usize) = map.next_value()?;
                                Ok(DataType::ArrayPrimitive(Box::new(pair.0), pair.1))
                            }
                            "ArrayComplex" => {
                                let pair: (DataType, usize) = map.next_value()?;
                                Ok(DataType::ArrayComplex(Box::new(pair.0), pair.1))
                            }
                            other => Err(serde::de::Error::unknown_field(other, &[])),
                        }
                    } else {
                        Err(serde::de::Error::custom("empty map for DataType"))
                    }
                }
            }
            deserializer.deserialize_any(V)
        } else {
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = DataType;

                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "compact DataType tuple")
                }

                fn visit_seq<A: serde::de::SeqAccess<'de>>(
                    self,
                    mut seq: A,
                ) -> Result<Self::Value, A::Error> {
                    // Read the first byte. If it equals the schema version marker
                    // (1), then the next byte is the tag. If it does not equal the
                    // version marker we assume older data that encoded only the
                    // tag as the first byte (backwards-compatibility).
                    let first: u8 = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::custom("missing tag/version"))?;
                    let tag: u8 = if first == 1u8 {
                        seq.next_element()?
                            .ok_or_else(|| serde::de::Error::custom("missing tag"))?
                    } else {
                        first
                    };
                    match tag {
                        0 => Ok(DataType::Null),
                        1 => Ok(DataType::Boolean),
                        2 => Ok(DataType::I8),
                        3 => Ok(DataType::U8),
                        4 => Ok(DataType::I16),
                        5 => Ok(DataType::U16),
                        6 => Ok(DataType::I32),
                        7 => Ok(DataType::U32),
                        8 => Ok(DataType::I64),
                        9 => Ok(DataType::U64),
                        10 => Ok(DataType::I128),
                        11 => Ok(DataType::U128),
                        12 => Ok(DataType::Isize),
                        13 => Ok(DataType::Usize),
                        14 => Ok(DataType::F32),
                        15 => Ok(DataType::F64),
                        16 => Ok(DataType::Char),
                        17 => {
                            let opt: Option<usize> = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::String(opt))
                        }
                        18 => {
                            let vars: Vec<(String, Option<DataType>)> = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::Enum(vars))
                        }
                        19 => {
                            let fields: Vec<Metadata> = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::Struct(fields))
                        }
                        20 => {
                            let inner: DataType = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::Option(Box::new(inner)))
                        }
                        21 => {
                            let items: Vec<DataType> = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::Tuple(items))
                        }
                        22 => {
                            let inner: DataType = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::VecPrimitive(Box::new(inner)))
                        }
                        23 => {
                            let inner: DataType = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::VecComplex(Box::new(inner)))
                        }
                        24 => {
                            let pair: (DataType, usize) = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::ArrayPrimitive(Box::new(pair.0), pair.1))
                        }
                        25 => {
                            let pair: (DataType, usize) = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::custom("missing payload"))?;
                            Ok(DataType::ArrayComplex(Box::new(pair.0), pair.1))
                        }
                        other => Err(serde::de::Error::custom(format!("unknown tag {}", other))),
                    }
                }
            }

            deserializer.deserialize_tuple(3, V)
        }
    }
}

// Implement Serialize/Deserialize for Metadata using derived-like behavior
impl serde::Serialize for Metadata {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            use serde::ser::SerializeMap;
            let mut m = serializer.serialize_map(Some(2))?;
            m.serialize_entry("name", &self.name)?;
            m.serialize_entry("dtype", &self.dtype)?;
            m.end()
        } else {
            use serde::ser::SerializeTuple;
            let mut t = serializer.serialize_tuple(2)?;
            t.serialize_element(&self.name)?;
            t.serialize_element(&self.dtype)?;
            t.end()
        }
    }
}

impl<'de> serde::Deserialize<'de> for Metadata {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = Metadata;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "Metadata")
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let name: String = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("missing name"))?;
                let dtype: DataType = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::custom("missing dtype"))?;
                Ok(Metadata { name, dtype })
            }

            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut map: M,
            ) -> Result<Self::Value, M::Error> {
                let mut name: Option<String> = None;
                let mut dtype: Option<DataType> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => name = Some(map.next_value()?),
                        "dtype" => dtype = Some(map.next_value()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                let name = name.ok_or_else(|| serde::de::Error::custom("missing name"))?;
                let dtype = dtype.ok_or_else(|| serde::de::Error::custom("missing dtype"))?;
                Ok(Metadata { name, dtype })
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_map(V)
        } else {
            deserializer.deserialize_tuple(2, V)
        }
    }
}
