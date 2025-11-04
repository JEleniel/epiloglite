use epiloglite_derive::record;
use flagset::FlagSet;

// record_flags has wrong generic parameter
#[record]
pub struct BadFlags {
    pub record_flags: FlagSet<u8>,
    pub name: String,
}

fn main() {}
