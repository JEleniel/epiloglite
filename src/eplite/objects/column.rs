//! A column in a table

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    name: String,
    primary_key: bool,
    primary_key_desc: bool,
    pk_conflict_clause: String,
    autoincrement: bool,
    nullable: bool,
    null_conflict_clause: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
    PrimaryKey {
        desc: bool,
        conflict_clause: String,
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
