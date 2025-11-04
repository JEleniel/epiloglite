use epiloglite_core::Record;
use epiloglite_core::RecordFlags;
use epiloglite_derive::record;
use flagset::FlagSet;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[record]
pub struct MyRecord {
    pub name: String,
    pub value: i32,
    pub record_id: u128,
    pub record_flags: FlagSet<RecordFlags>,
}

fn main() {
    let mut r = MyRecord {
        name: "x".into(),
        value: 1,
        record_id: 0u128,
        record_flags: FlagSet::empty(),
    };
    assert_eq!(r.record_id(), 0u128);
    r.set_record_id(42);
    assert_eq!(r.record_id(), 42u128);
    let _flags: &FlagSet<RecordFlags> = r.flags();
}
