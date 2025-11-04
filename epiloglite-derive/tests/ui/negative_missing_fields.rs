use epiloglite_derive::record;

// This test attempts to use the macro on a struct that already provides a
// record_id with wrong type and missing record_flags; the macro should ensure
// proper types or fail depending on the mismatch. This file expects a compile
// error because record_id is not u128.

#[record]
pub struct BadId {
    pub record_id: u64, // wrong type, should trigger panic in macro expansion
    pub name: String,
}

fn main() {}
