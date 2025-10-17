// This file has been deleted.
use crate::crc::calculate_crc;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct TestStruct {
    a: u32,
    b: String,
}

#[test]
fn test_calculate_crc_different_inputs() {
    let v1 = TestStruct {
        a: 1,
        b: "x_longer_string_12345".to_string(),
    };
    let v2 = TestStruct {
        a: 2,
        b: "x_longer_string_12345".to_string(),
    };
    let v3 = TestStruct {
        a: 1,
        b: "y_different_longer_string_67890".to_string(),
    };
    let crc1 = calculate_crc(&v1);
    let crc2 = calculate_crc(&v2);
    let crc3 = calculate_crc(&v3);
    assert_ne!(crc1, crc2, "CRC should differ for different input");
    assert_ne!(crc1, crc3, "CRC should differ for different input");
    assert_ne!(crc2, crc3, "CRC should differ for different input");
}

#[test]
fn test_calculate_crc_empty_struct() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    struct EmptyStruct;
    let value = EmptyStruct;
    let crc = calculate_crc(&value);
    // For empty struct, bincode encoding is very short, so CRC will be 0
    assert_eq!(crc, 0, "CRC should be zero for empty struct");
}

#[test]
fn test_calculate_crc_small_struct() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    struct SmallStruct(u8);
    let value = SmallStruct(1);
    let crc = calculate_crc(&value);
    // For very small struct, bincode encoding may be < 4 bytes, so CRC is 0
    assert_eq!(
        crc, 0,
        "CRC should be zero for struct with encoding < 4 bytes"
    );
}

#[test]
fn test_calculate_crc_security() {
    // Malformed struct: simulate by using a struct with a large string
    let value = TestStruct {
        a: 0,
        b: "\0".repeat(1000),
    };
    let crc = calculate_crc(&value);
    assert_ne!(crc, 0, "CRC should not be zero for large but valid input");
}
