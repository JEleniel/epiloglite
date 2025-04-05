use crate::parser::literals::Level2Name;

pub struct ReindexStatement {
    reindex_type: ReindexType,
}

pub enum ReindexType {
    General,
    Collation(String),
    TableOrIndex(Level2Name),
}
