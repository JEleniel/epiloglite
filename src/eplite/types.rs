pub mod index;
pub mod orderby;
pub mod virtualtable;

#[derive(Debug, Clone)]
pub struct Value {
    value: ValueType,
    changed: bool,
    bound: bool,
}

impl Value {
    pub fn changed(self: &Self) -> bool {
        self.changed
    }

    pub fn bound(self: &Self) -> bool {
        self.bound
    }
}

#[derive(Debug, Clone)]
pub enum ValueType {
    blob(Vec<u8>),
    bytes(Vec<u8>),
    double(f64),
    i32(i32),
    i64(i64),
    string_utf8(String),
    string_utf16(Vec<u8>),
    string_utf16le(Vec<u8>),
    string_utf16be(Vec<u8>),
}
