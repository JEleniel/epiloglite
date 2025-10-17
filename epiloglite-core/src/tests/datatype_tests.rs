// This file has been deleted.
use crate::datatype::DataType;

// All tests have been removed after migration.

#[test]
fn test_is_float() {
    assert!(DataType::F32.is_float());
    assert!(DataType::F64.is_float());
    assert!(!DataType::I8.is_float());
    assert!(!DataType::Null.is_float());
}

#[test]
fn test_is_integer() {
    assert!(DataType::I8.is_integer());
    assert!(DataType::U8.is_integer());
    assert!(DataType::I16.is_integer());
    assert!(DataType::U16.is_integer());
    assert!(DataType::I32.is_integer());
    assert!(DataType::U32.is_integer());
    assert!(DataType::I64.is_integer());
    assert!(DataType::U64.is_integer());
    assert!(DataType::I128.is_integer());
    // Not integer
    assert!(!DataType::U128.is_integer());
    assert!(!DataType::F32.is_integer());
    assert!(!DataType::Null.is_integer());
}

#[test]
fn test_string_variant() {
    let dt = DataType::String(Some(255));
    if let DataType::String(Some(len)) = dt {
        assert_eq!(len, 255);
    } else {
        panic!("Expected String(Some(255))");
    }
    let dt2 = DataType::String(None);
    if let DataType::String(None) = dt2 {
        // ok
    } else {
        panic!("Expected String(None)");
    }
}

#[test]
fn test_byte_array_variant() {
    let dt = DataType::ByteArray;
    assert_eq!(dt, DataType::ByteArray);
}

#[test]
fn test_equality_and_clone() {
    let a = DataType::I32;
    let b = a.clone();
    assert_eq!(a, b);
    let s1 = DataType::String(Some(10));
    let s2 = s1.clone();
    assert_eq!(s1, s2);
}

#[test]
fn test_debug_format() {
    let dt = DataType::F64;
    let s = format!("{:?}", dt);
    assert!(s.contains("F64"));
}
