use epiloglite_core::RecordFlags;
use epiloglite_derive::collection;
use flagset::FlagSet;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Dummy CInt for test
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CInt(pub i64);

#[collection]
pub struct TestRecord {
    pub name: String,
}

#[test]
fn test_record_id_added() {
    let rec = TestRecord {
        record_id: CInt(1),
        record_flags: FlagSet::empty(),
        name: "foo".to_string(),
    };
    assert_eq!(rec.record_id, CInt(1)); // Ensure record_id is included
}

#[test]
fn test_container_metadata_readonly() {
    let meta = TestRecordCollection::metadata();
    assert_eq!(meta.len(), 1); // Only 'name' is present in metadata
    assert_eq!(meta[0].name, "name");
}

#[test]
fn test_container_struct() {
    let rec = TestRecord {
        record_id: CInt(2),
        record_flags: FlagSet::empty(),
        name: "bar".to_string(),
    };
    let cont = TestRecordCollection {
        // Ensure all fields are included
        container_id: CInt(10),
        name: "container".to_string(),
        records: vec![rec.clone()],
    };
    assert_eq!(cont.records[0].name, "bar");
    assert_eq!(cont.container_id, CInt(10));
    let meta = TestRecordCollection::metadata();
    assert_eq!(meta[0].name, "name");
}

#[collection]
pub struct WithId {
    pub record_id: CInt,
    pub value: i32,
}

#[test]
fn test_existing_record_id() {
    let rec = WithId {
        record_id: CInt(5),
        record_flags: FlagSet::empty(),
        value: 42,
    };
    assert_eq!(rec.record_id, CInt(5)); // Ensure record_id is included
}
