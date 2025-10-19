use epiloglite_core::RecordFlags;
use epiloglite_derive::data_field;
use flagset::FlagSet;

// Mirror the trait expected by the derive macro. We keep it local to the test
// so that the macro's `impl RecordTrait for ...` resolves to this definition.
pub trait RecordTrait {
    fn id(&self) -> CInt;
    fn set_id(&mut self, id: CInt);
    fn flags(&self) -> &FlagSet<RecordFlags>;
    fn flags_mut(&mut self) -> &mut FlagSet<RecordFlags>;
}
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// Dummy CInt for test
#[derive(Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CInt(pub i64);

#[data_field]
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

#[data_field]
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
