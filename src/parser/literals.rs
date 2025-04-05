use chrono::{DateTime, Utc};

pub struct QualifiedTableName {
    pub table_name: Level2Name,
    pub as_name: Option<String>,
    pub indexed: bool,
    pub indexed_by_name: Option<String>,
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

pub struct TypeName {
    pub type_name: StorageTypeName,
    pub min: f64,
    pub max: f64,
}

pub enum StorageTypeName {
    Null,
    Boolean,
    Integer,
    Real,
    Text,
    Blob,
    DateTime,
}
