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
    Blob(Vec<u8>),
    Bytes(Vec<u8>),
    Double(f64),
    I32(i32),
    I64(i64),
    StringUtf8(String),
    StringUtf16(Vec<u8>),
    StringUtf16Le(Vec<u8>),
    StringUtf16Be(Vec<u8>),
}
