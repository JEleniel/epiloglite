use crate::parser::{
    clauses::{CommonTableExpression, ConflictAction, ReturningClause, UpsertClause},
    expression::Expression,
    literals::Level2Name,
};

use super::select::SelectStatment;

pub struct InsertStatement {
    recursive: bool,
    common_table_expressions: Option<Vec<CommonTableExpression>>,
    action: InsertAction,
    tabble_name: Level2Name,
    alias: Option<String>,
    column_names: Option<Vec<String>>,
    insert_source: InsertSource,
    returning_clause: Option<ReturningClause>,
}

pub enum InsertAction {
    Replace,
    Insert(Option<ConflictAction>),
}

pub enum InsertSource {
    Values(Vec<Expression>, Option<UpsertClause>),
    Select(SelectStatment, Option<UpsertClause>),
    Default,
}
