use super::{expression::Expression, literals::QualifiedTableName};

pub struct UnaryOperator {
    operation: UnaryOperation,
    expr: Expression,
}

pub enum UnaryOperation {
    Group,
    Not,
    Positive,
    Negative,
}

pub struct CollateOperator {
    expr: Expression,
    collation: CollationName,
}

pub enum CollationName {
    Binary,
    NoCase,
    RTrim,
}

pub struct BinaryOperator {
    pub operation: BinaryOperation,
    left_expr: Expression,
    right_expr: Expression,
}

pub enum BinaryOperation {
    And,
    Or,
    ShiftLeft,
    ShiftRight,
    Collate,
    Extract,
    Multiply,
    Divide,
    Modulus,
    Add,
    Subtract,
    GreaterThan,
    LessThan,
    Equals,
    GreaterThanOrEquals,
    LessThanOrEquals,
}

pub struct TernaryOperator {
    operation: TernaryOperation,
    expr1: Expression,
    expr2: Expression,
    expr3: Expression,
}

pub enum TernaryOperation {
    Between,
}

pub struct InOperator {
    not: bool,
}

pub enum InValues {
    Select(SelectStatement),
    QualifiedTableName(QualifiedTableName),
    TableFunction(TableFunction),
}
