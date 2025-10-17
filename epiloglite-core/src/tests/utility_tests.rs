// This file has been deleted.
use crate::serialized_size;
use crate::{SerializeError, calculate_crc, try_from_reader, try_from_slice, try_to_writer};
use bincode::de::read::SliceReader;
use bincode::enc::write::SliceWriter;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
struct TestStruct {
    a: u32,
    b: String,
}
#[test]
fn test_try_to_writer_and_try_from_reader_success() {
    let value = TestStruct {
        a: 7,
        b: "world".to_string(),
    };
    let mut buf = [0u8; 128];
    let mut writer = SliceWriter::new(&mut buf[..]);
    try_to_writer(&value, &mut writer).expect("encode to writer");
    let written = writer.bytes_written();
    let mut reader = SliceReader::new(&buf[..written]);
    let decoded: TestStruct = try_from_reader(&mut reader).expect("decode from reader");
    assert_eq!(value, decoded);
}

#[test]
fn test_try_from_slice_invalid_data() {
    // Not a valid bincode encoding for TestStruct
    let bad_data = [0u8, 1, 2, 3, 4, 5];
    let result: Result<TestStruct, _> = try_from_slice(&bad_data);
    assert!(result.is_err());
}

#[test]
fn test_try_from_reader_invalid_data() {
    let bad_data = [0u8, 1, 2, 3, 4, 5];
    let mut reader = SliceReader::new(&bad_data);
    let result: Result<TestStruct, _> = try_from_reader(&mut reader);
    assert!(result.is_err());
}

#[test]
fn test_try_to_writer_and_try_from_reader_partial_read() {
    // Write two structs, read both, ensure correct
    let value1 = TestStruct {
        a: 1,
        b: "a".to_string(),
    };
    let value2 = TestStruct {
        a: 2,
        b: "b".to_string(),
    };
    let mut buf = [0u8; 256];
    let mut writer = SliceWriter::new(&mut buf[..]);
    try_to_writer(&value1, &mut writer).unwrap();
    try_to_writer(&value2, &mut writer).unwrap();
    let written = writer.bytes_written();
    let mut reader = SliceReader::new(&buf[..written]);
    let decoded1: TestStruct = try_from_reader(&mut reader).unwrap();
    let decoded2: TestStruct = try_from_reader(&mut reader).unwrap();
    assert_eq!(decoded1, value1);
    assert_eq!(decoded2, value2);
}

#[test]
fn test_try_into_vec_and_try_from_slice_security() {
    // Use a truncated buffer that cannot represent a valid TestStruct
    let bad_data = vec![0u8; 2]; // Too short for u32 + string
    let result: Result<TestStruct, _> = try_from_slice(&bad_data);
    assert!(result.is_err());
}

#[test]
fn test_try_into_vec_and_try_from_slice_empty() {
    let result: Result<TestStruct, _> = try_from_slice(&[]);
    assert!(result.is_err());
}

#[test]
fn test_try_into_vec_roundtrip_and_serialized_size() {
    let value = TestStruct {
        a: 42,
        b: "hello world".to_string(),
    };
    // encode to vec
    let vec = crate::try_into_vec(&value).expect("encode to vec");
    // decode from slice should roundtrip
    let decoded: TestStruct = try_from_slice(&vec).expect("decode from vec");
    assert_eq!(value, decoded);

    // serialized_size should equal vec.len()
    let size = serialized_size(&value).expect("serialized_size");
    assert_eq!(size as usize, vec.len());
}

#[test]
fn test_try_from_slice_decode_error_variant() {
    // random bytes that are unlikely to decode
    let bad_data = vec![0xffu8; 10];
    let result: Result<TestStruct, _> = try_from_slice(&bad_data);
    match result {
        Err(crate::SerializeError::Decode(_)) => {}
        other => panic!("expected Decode error, got: {:?}", other),
    }
}

#[test]
fn test_try_to_writer_encode_error() {
    // Use a tiny buffer so encoding will fail when writing the full struct
    let value = TestStruct {
        a: 1,
        b: "this is a longer string that won't fit".to_string(),
    };
    let mut buf = [0u8; 1];
    let mut writer = SliceWriter::new(&mut buf[..]);
    let result = try_to_writer(&value, &mut writer);
    match result {
        Err(crate::SerializeError::Encode(_)) => {}
        other => panic!("expected Encode error, got: {:?}", other),
    }
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
    let crc1 = calculate_crc(&v1).expect("crc1");
    let crc2 = calculate_crc(&v2).expect("crc2");
    let crc3 = calculate_crc(&v3).expect("crc3");
    assert_ne!(crc1, crc2, "CRC should differ for different input");
    assert_ne!(crc1, crc3, "CRC should differ for different input");
    assert_ne!(crc2, crc3, "CRC should differ for different input");
}

#[test]
fn test_calculate_crc_empty_struct() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    struct EmptyStruct;
    let value = EmptyStruct;
    let result = calculate_crc(&value);
    // For empty struct the encoded size is < 4 bytes, so CRC calculation should return CrcInputEmpty
    match result {
        Err(SerializeError::CrcInputEmpty) => {}
        other => panic!("expected CrcInputEmpty, got: {:?}", other),
    }
}

#[test]
fn test_calculate_crc_small_struct() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    struct SmallStruct(u8);
    let value = SmallStruct(1);
    let result = calculate_crc(&value);
    match result {
        Err(SerializeError::CrcInputEmpty) => {}
        other => panic!("expected CrcInputEmpty, got: {:?}", other),
    }
}

#[test]
fn test_calculate_crc_security() {
    // Malformed struct: simulate by using a struct with a large string
    let value = TestStruct {
        a: 0,
        b: "\0".repeat(1000),
    };
    let crc = calculate_crc(&value).expect("calculate_crc");
    assert_ne!(crc, 0, "CRC should not be zero for large but valid input");
}
