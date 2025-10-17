// This file is not needed and will be removed.
use crate::CInt;
use crate::eplite::persistence::{OffsetPointer, RowIdIndex};

#[test]
fn test_new_index_is_empty() {
    let idx = RowIdIndex::new();
    assert_eq!(idx.len(), 0);
    assert_eq!(idx.get(&CInt::from(1)), None);
}

// Additional tests would follow...
use crate::CInt;
use crate::eplite::persistence::{OffsetPointer, RowIdIndex};

#[test]
fn test_new_index_is_empty() {
    let idx = RowIdIndex::new();
    assert_eq!(idx.len(), 0);
    assert_eq!(idx.get(&CInt::from(1)), None);
}

#[test]
fn test_insert_and_get() {
    let mut idx = RowIdIndex::new();
    let ptr = OffsetPointer {
        page_id: CInt::from(2),
        offset: CInt::from(3),
    };
    idx.insert(CInt::from(1), ptr.clone());
    assert_eq!(idx.get(&CInt::from(1)), Some(&ptr));
}

#[test]
fn test_remove_sets_null_pointer() {
    let mut idx = RowIdIndex::new();
    let ptr = OffsetPointer {
        page_id: CInt::from(2),
        offset: CInt::from(3),
    };
    idx.insert(CInt::from(1), ptr);
    idx.remove(&CInt::from(1));
    assert_eq!(idx.get(&CInt::from(1)), Some(&OffsetPointer::null()));
}

#[test]
fn test_get_entries_sorted() {
    let mut idx = RowIdIndex::new();
    idx.insert(
        CInt::from(2),
        OffsetPointer {
            page_id: CInt::from(2),
            offset: CInt::from(3),
        },
    );
    idx.insert(
        CInt::from(1),
        OffsetPointer {
            page_id: CInt::from(1),
            offset: CInt::from(2),
        },
    );
    let entries = idx.get_entries();
    assert_eq!(entries[0].0, CInt::from(1));
    assert_eq!(entries[1].0, CInt::from(2));
}

#[test]
fn test_serialize_deserialize_roundtrip() {
    let mut idx = RowIdIndex::new();
    idx.insert(
        CInt::from(1),
        OffsetPointer {
            page_id: CInt::from(2),
            offset: CInt::from(3),
        },
    );
    let bytes: Vec<u8> = (&idx).try_into().unwrap();
    let decoded = RowIdIndex::try_from(bytes.as_slice()).unwrap();
    assert_eq!(idx, decoded);
}

#[test]
fn test_try_from_invalid_bytes() {
    let bytes = vec![0u8; 2];
    let result = RowIdIndex::try_from(bytes.as_slice());
    assert!(result.is_err());
}
