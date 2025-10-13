use serde_json;
#[test]
fn test_rowidindex_serde_roundtrip() {
    let mut index = RowIdIndex::new();
    index.insert(CInt::from(1), make_pointer(10, 20));
    index.insert(CInt::from(2), make_pointer(30, 40));
    index.insert(CInt::from(3), OffsetPointer::null());
    let json = serde_json::to_string(&index).expect("serialize");
    let de: RowIdIndex = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(index, de);
}
//! Tests for RowIdIndex in indexentry.rs

use crate::CInt;
use crate::eplite::persistence::OffsetPointer;
use crate::eplite::persistence::rowidindex::RowIdIndex;

/// Helper to create an OffsetPointer with CInt fields.
fn make_pointer(offset: u64, len: u32) -> OffsetPointer {
    OffsetPointer {
        page_number: CInt::from(offset),
        offset: CInt::from(len),
    }
}

#[test]
fn test_new_index_is_empty() {
    let index = RowIdIndex::new();
    assert_eq!(index.len(), 0);
    assert!(index.get(&CInt::from(1)).is_none());
}

#[test]
fn test_insert_and_get() {
    let mut index = RowIdIndex::new();
    let ptr = make_pointer(42, 8);
    index.insert(CInt::from(1), ptr.clone());
    assert_eq!(index.len(), 1);
    assert_eq!(index.get(&CInt::from(1)), Some(&ptr));
}

#[test]
fn test_insert_overwrite() {
    let mut index = RowIdIndex::new();
    let ptr1 = make_pointer(10, 4);
    let ptr2 = make_pointer(20, 8);
    index.insert(CInt::from(1), ptr1.clone());
    index.insert(CInt::from(1), ptr2.clone());
    assert_eq!(index.len(), 1);
    assert_eq!(index.get(&CInt::from(1)), Some(&ptr2));
}

#[test]
fn test_remove_sets_null_pointer() {
    let mut index = RowIdIndex::new();
    let ptr = make_pointer(100, 16);
    index.insert(CInt::from(5), ptr);
    index.remove(&CInt::from(5));
    assert_eq!(index.len(), 1);
    let removed = index.get(&CInt::from(5)).unwrap();
    assert_eq!(removed, &OffsetPointer::null());
}

#[test]
fn test_get_entries_sorted() {
    let mut index = RowIdIndex::new();
    let ptr1 = make_pointer(1, 1);
    let ptr2 = make_pointer(2, 2);
    let ptr3 = make_pointer(3, 3);
    index.insert(CInt::from(20), ptr2.clone());
    index.insert(CInt::from(10), ptr1.clone());
    index.insert(CInt::from(30), ptr3.clone());
    let entries = index.get_entries();
    assert_eq!(
        entries,
        vec![
            (CInt::from(10), ptr1),
            (CInt::from(20), ptr2),
            (CInt::from(30), ptr3)
        ]
    );
}

#[test]
fn test_len_after_multiple_operations() {
    let mut index = RowIdIndex::new();
    assert_eq!(index.len(), 0);
    index.insert(CInt::from(1), make_pointer(1, 1));
    index.insert(CInt::from(2), make_pointer(2, 2));
    assert_eq!(index.len(), 2);
    index.remove(&CInt::from(1));
    assert_eq!(index.len(), 2); // remove does not delete, just sets null
}

#[test]
fn test_get_nonexistent_returns_none() {
    let index = RowIdIndex::new();
    assert!(index.get(&CInt::from(999)).is_none());
}

#[test]
fn test_remove_nonexistent_entry_creates_null_entry() {
    let mut index = RowIdIndex::new();
    assert!(index.get(&CInt::from(42)).is_none());
    index.remove(&CInt::from(42));
    let ptr = index.get(&CInt::from(42)).unwrap();
    assert_eq!(ptr, &OffsetPointer::null());
}

#[test]
fn test_clone_and_eq() {
    let mut index1 = RowIdIndex::new();
    index1.insert(CInt::from(1), make_pointer(1, 1));
    let index2 = index1.clone();
    assert_eq!(index1, index2);
}

#[test]
fn test_partial_eq_with_different_entries() {
    let mut index1 = RowIdIndex::new();
    let mut index2 = RowIdIndex::new();
    index1.insert(CInt::from(1), make_pointer(1, 1));
    index2.insert(CInt::from(2), make_pointer(2, 2));
    assert_ne!(index1, index2);
}
