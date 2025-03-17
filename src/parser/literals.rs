use chrono::{DateTime, Utc};

pub struct QualifiedTableName {
    table_name: Level2Name,
    as_name: Option<String>,
    indexed: bool,
    indexed_by_name: Option<String>,
}

pub struct Level3Name {
    pub level_2_name: Option<Level2Name>,
    pub object_name: String,
}

pub struct Level2Name {
    pub schema_name: Option<String>,
    pub container_name: String,
}

pub enum Literal {
    String(String),
    Real(f64),
    Integer(i64),
    Blob(Vec<u8>),
    DateTime(DateTime<Utc>),
    Null,
}

pub enum TypeName {
    Text,
    Real,
    Integer,
    Blob,
    DateTime,
    Null,
}
