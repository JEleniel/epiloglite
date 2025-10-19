use epiloglite_core::RecordFlags;
use epiloglite_derive::data_field;
use flagset::FlagSet;

pub trait RecordTrait {
    fn id(&self) -> CInt;
    fn set_id(&mut self, id: CInt);
    fn flags(&self) -> &FlagSet<RecordFlags>;
    fn flags_mut(&mut self) -> &mut FlagSet<RecordFlags>;
}
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CInt(pub i64);

#[data_field]
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
