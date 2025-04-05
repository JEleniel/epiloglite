use crate::parser::{expression::Expression, literals::QualifiedTableName};

use super::{super::statements::select::SelectStatment, general_functions::Function};

pub enum Operator {
    Collate(CollateOperator),
    In(InOperator),
}

pub struct CollateOperator {
    pub expr: Box<Expression>,
    pub collation: CollationName,
}

pub enum CollationName {
    Binary,
    NoCase,
    RTrim,
}

pub struct InOperator {
    pub expr: Box<Expression>,
    pub values: InValues,
}

pub enum InValues {
    Select(Box<SelectStatment>),
    QualifiedTableName(QualifiedTableName),
    TableFunction(Function),
}
