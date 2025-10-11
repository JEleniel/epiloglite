//! A column in an EL table

use crate::{DataType, eplite::sql::ConflictClause};

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    name: String,
    data_type: DataType,
    autoincrement: bool,
    constraints: Vec<ColumnConstraint>,
    default_value: Option<DefaultValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
    PrimaryKey {
        desc: bool,
        conflict_clause: ConflictClause,
    },
    Nullable {
        conflict_clause: String,
    },
    Unique {
        conflict_clause: String,
    },
    Check {
        expresion: String,
    },
    Default(DefaultValue),
    Collate(String),
    ForeignKey(String),
    Generated {
        expresion: String,
        generation_type: GenerationType,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefaultValue {
    Expression(String),
    Literal(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenerationType {
    None,
    Stored,
    Virtual,
}
