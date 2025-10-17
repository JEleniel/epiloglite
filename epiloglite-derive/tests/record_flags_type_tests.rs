use epiloglite_core::RecordFlags;
use epiloglite_derive::collection;
use flagset::FlagSet;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CInt(pub i64);

#[collection]
pub struct FlagTest {
    pub value: i32,
}

#[test]
fn test_record_flags_type() {
    let rec = FlagTest {
        record_id: CInt(1),
        record_flags: FlagSet::empty(),
        value: 99,
    };
    assert_eq!(rec.record_flags, FlagSet::empty());
}
