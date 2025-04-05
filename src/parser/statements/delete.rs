use crate::parser::{
    clauses::{CommonTableExpression, ReturningClause},
    expression::Expression,
    literals::QualifiedTableName,
};

pub struct DeleteStatement {
    recursive: bool,
    common_table_expressions: Option<Vec<CommonTableExpression>>,
    qualified_table_name: QualifiedTableName,
    where_expr: Option<Expression>,
    returning_clause: Option<ReturningClause>,
}
