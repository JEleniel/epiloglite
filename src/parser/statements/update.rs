use crate::parser::{
    clauses::{CommonTableExpression, ConflictAction, FromClause, ReturningClause, SetClause},
    expression::Expression,
    literals::QualifiedTableName,
};

pub struct UpdateStatement {
    recursive: bool,
    common_table_statements: Vec<CommonTableExpression>,
    conflice_action: Option<ConflictAction>,
    table_name: QualifiedTableName,
    set: Vec<SetClause>,
    from: FromClause,
    where_expr: Option<Expression>,
    returning_clause: Option<ReturningClause>,
}
