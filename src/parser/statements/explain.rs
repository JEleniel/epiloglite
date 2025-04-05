use super::SqlStatement;

pub struct ExplainStatement {
    statement: Box<SqlStatement>,
}
