#[test]
fn test_builder_methods() {
    use crate::eplite::persistence::offsetpointer::OffsetPointer;
    let mut header = DatabaseHeader::default();
    header
        .with_signature("SIG".to_string())
        .with_format_version(42)
        .with_page_size_exponent(13)
        .with_flags(0xAA)
        .with_freelist_offset(OffsetPointer {
            page_id: CInt::from(2),
            offset: CInt::from(1234),
        })
        .with_application_id(CInt::from(0x12345678))
        .with_migration_version(CInt::from(0x87654321))
        .with_crc(0xCAFEBABE);
    assert_eq!(header.signature, "SIG");
    assert_eq!(header.format_version, 42);
    assert_eq!(header.page_size_exponent, 13);
    assert_eq!(header.flags, 0xAA);
    assert_eq!(header.freelist_offset.page_id, CInt::from(2));
    assert_eq!(header.freelist_offset.offset, CInt::from(1234));
    assert_eq!(header.application_id, CInt::from(0x12345678));
    assert_eq!(header.migration_version, CInt::from(0x87654321));
    assert_eq!(header.crc, 0xCAFEBABE);
}
// This file is not needed and will be removed.
use crate::CInt;
use crate::calculate_crc;
use crate::eplite::persistence::{
    CURRENT_FORMAT_VERSION, DEFAULT_PAGE_SIZE_EXPONENT, DatabaseHeader, EPLITE_SIGNATURE,
    HeaderError, MAX_HEADER_SIZE, MIN_HEADER_SIZE,
};

#[test]
fn test_default_header_is_valid() {
    let header = DatabaseHeader::default();
    assert_eq!(header.signature, EPLITE_SIGNATURE);
    assert_eq!(header.format_version, CURRENT_FORMAT_VERSION);
    assert_eq!(header.page_size_exponent, DEFAULT_PAGE_SIZE_EXPONENT);
    assert_eq!(header.freelist_offset.offset, CInt::from(MAX_HEADER_SIZE));
    assert_eq!(header.freelist_offset.page_id, CInt::zero());
    assert!(header.validate().is_ok());
}

// Additional tests would follow...
use crate::CInt;
use crate::calculate_crc;
use crate::eplite::persistence::{
    CURRENT_FORMAT_VERSION, DEFAULT_PAGE_SIZE_EXPONENT, DatabaseHeader, EPLITE_SIGNATURE,
    HeaderError, MAX_HEADER_SIZE, MIN_HEADER_SIZE,
};

#[test]
fn test_default_header_is_valid() {
    let header = DatabaseHeader::default();
    assert_eq!(header.signature, EPLITE_SIGNATURE);
    assert_eq!(header.format_version, CURRENT_FORMAT_VERSION);
    assert_eq!(header.page_size_exponent, DEFAULT_PAGE_SIZE_EXPONENT);
    assert_eq!(header.freelist_offset.offset, CInt::from(MAX_HEADER_SIZE));
    assert_eq!(header.freelist_offset.page_id, CInt::zero());
    assert!(header.validate().is_ok());
}

#[test]
fn test_page_size_calculation() {
    let mut header = DatabaseHeader::default();
    header.page_size_exponent = 12;
    assert_eq!(header.page_size(), 4096);
    header.page_size_exponent = 9;
    assert_eq!(header.page_size(), 512);
}

#[test]
fn test_serialize_deserialize_roundtrip() {
    let header = DatabaseHeader::default();
    let bytes: Vec<u8> = (&header).try_into().unwrap();
    let decoded = DatabaseHeader::try_from(bytes.as_slice()).unwrap();
    assert_eq!(header, decoded);
}

#[test]
fn test_invalid_signature() {
    let mut header = DatabaseHeader::default();
    header.signature = "INVALID".to_string();
    header.crc = calculate_crc(&header);
    let result = header.validate();
    assert!(matches!(
        result,
        Err(HeaderError::InvalidHeaderSignature(_))
    ));
}

#[test]
fn test_format_too_new() {
    let mut header = DatabaseHeader::default();
    header.format_version = CURRENT_FORMAT_VERSION + 1;
    header.crc = calculate_crc(&header);
    let result = header.validate();
    assert!(matches!(result, Err(HeaderError::FormatTooNew(_))));
}

#[test]
fn test_invalid_page_size() {
    let mut header = DatabaseHeader::default();
    header.page_size_exponent = 8; // Below valid range
    header.crc = calculate_crc(&header);
    let result = header.validate();
    assert!(matches!(result, Err(HeaderError::InvalidPageSize(_))));
}

#[test]
fn test_invalid_freelist_offset() {
    let mut header = DatabaseHeader::default();
    header.freelist_offset.page_id = CInt::from(1);
    header.crc = calculate_crc(&header);
    let result = header.validate();
    assert!(matches!(
        result,
        Err(HeaderError::InvalidFreelistOffset(_, _, _))
    ));
}

#[test]
fn test_invalid_crc() {
    let mut header = DatabaseHeader::default();
    header.crc = 0xDEADBEEF;
    let result = header.validate();
    assert!(matches!(result, Err(HeaderError::InvalidCRC(_, _))));
}

#[test]
fn test_try_from_invalid_size() {
    let bytes = vec![0u8; MIN_HEADER_SIZE - 1];
    let result = DatabaseHeader::try_from(bytes.as_slice());
    assert!(matches!(result, Err(HeaderError::InvalidSize(_, _, _))));
    let bytes = vec![0u8; MAX_HEADER_SIZE + 1];
    let result = DatabaseHeader::try_from(bytes.as_slice());
    assert!(matches!(result, Err(HeaderError::InvalidSize(_, _, _))));
}
