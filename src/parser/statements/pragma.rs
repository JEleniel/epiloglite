use crate::parser::literals::{Level2Name, Literal};

pub struct PragmaStatement {
    name: Level2Name,
    value: PragmaValue,
}

pub enum PragmaValue {
    Name(String),
    Literal(Literal),
}
