// This file is not needed and will be removed.
use crate::CInt;
use crate::eplite::persistence::{MetadataEntry, OffsetPointer};

#[test]
fn test_table_variant() {
    let entry = MetadataEntry::Table {
        table_id: CInt::from(1),
        name: "test_table".to_string(),
        first_page: OffsetPointer {
            page_id: CInt::from(2),
            offset: CInt::from(3),
        },
        crc: 0x12345678,
    };
    // Additional tests would follow...
}
use crate::CInt;
use crate::eplite::persistence::{MetadataEntry, OffsetPointer};

#[test]
fn test_table_variant() {
    let entry = MetadataEntry::Table {
        table_id: CInt::from(1),
        name: "test_table".to_string(),
        first_page: OffsetPointer {
            page_id: CInt::from(2),
            offset: CInt::from(3),
        },
        crc: 0x12345678,
    };
    if let MetadataEntry::Table {
        table_id,
        name,
        first_page,
        crc,
    } = entry
    {
        assert_eq!(table_id, CInt::from(1));
        assert_eq!(name, "test_table");
        assert_eq!(first_page.page_id, CInt::from(2));
        assert_eq!(first_page.offset, CInt::from(3));
        assert_eq!(crc, 0x12345678);
    } else {
        panic!("Not a Table variant");
    }
}

#[test]
fn test_index_variant() {
    let entry = MetadataEntry::Index {
        index_id: CInt::from(1),
        table_id: CInt::from(2),
        name: "idx".to_string(),
        index_def: vec![1, 2, 3],
        crc: 0x87654321,
    };
    if let MetadataEntry::Index {
        index_id,
        table_id,
        name,
        index_def,
        crc,
    } = entry
    {
        assert_eq!(index_id, CInt::from(1));
        assert_eq!(table_id, CInt::from(2));
        assert_eq!(name, "idx");
        assert_eq!(index_def, vec![1, 2, 3]);
        assert_eq!(crc, 0x87654321);
    } else {
        panic!("Not an Index variant");
    }
}

#[test]
fn test_view_variant() {
    let entry = MetadataEntry::View {
        view_id: CInt::from(1),
        name: "view1".to_string(),
        view_def: vec![4, 5, 6],
        crc: 0xABCDEF01,
    };
    if let MetadataEntry::View {
        view_id,
        name,
        view_def,
        crc,
    } = entry
    {
        assert_eq!(view_id, CInt::from(1));
        assert_eq!(name, "view1");
        assert_eq!(view_def, vec![4, 5, 6]);
        assert_eq!(crc, 0xABCDEF01);
    } else {
        panic!("Not a View variant");
    }
}

#[test]
fn test_serialize_deserialize_roundtrip() {
    let entry = MetadataEntry::Table {
        table_id: CInt::from(1),
        name: "test_table".to_string(),
        first_page: OffsetPointer {
            page_id: CInt::from(2),
            offset: CInt::from(3),
        },
        crc: 0x12345678,
    };
    let bytes: Vec<u8> = (&entry).try_into().unwrap();
    let decoded = MetadataEntry::try_from(bytes.as_slice()).unwrap();
    assert_eq!(entry, decoded);
}

#[test]
fn test_try_from_invalid_bytes() {
    let bytes = vec![0u8; 2];
    let result = MetadataEntry::try_from(bytes.as_slice());
    assert!(result.is_err());
}
