use crate::parser::{
    clauses::{CommonTableExpression, FromClause, OrderingTerm},
    expression::Expression,
};

pub struct SelectStatment {
    recursive: bool,
    common_table_expressions: Option<Vec<CommonTableExpression>>,
    select_clause: SelectClause,
    ordering_terms: Vec<OrderingTerm>,
    limit: Option<LimitType>,
}

pub enum SelectClause {
    Values(Vec<Expression>, Option<Box<CompoundOperator>>),
    SimpleSelect(SimpleSelectStatement, Option<Box<CompoundOperator>>),
}

pub struct SimpleSelectStatement {
    distinct_all: DistinctAll,
    result_columns: Vec<ResultColumn>,
    from: Option<FromClause>,
    where_expr: Option<Expression>,
    group_by: Option<Vec<Expression>>,
    having: Expression,
    windows: Vec<Window>,
    values: Vec<Vec<Expression>>,
}

pub enum DistinctAll {
    None,
    Distinct,
    All,
}

pub enum ResultColumn {
    Expression(Expression, Option<String>),
    All,
    TableAll(String),
}

pub struct Window {
    name: String,
    window_def: WindowDef,
}

pub struct WindowDef {
    base_name: Option<String>,
    partition_by: Vec<Expression>,
    order_by: Vec<OrderingTerm>,
    frame_spec: Option<FrameSpec>,
}

pub struct FrameSpec {
    scope: FrameScope,
    frame_type: FrameType,
    exclude: FrameExclude,
}

pub enum FrameScope {
    Range,
    Rows,
    Groups,
}

pub enum FrameType {
    Between(FrameBetween, FrameBetween),
    UnboundedPreceeding,
    Preceeding(Expression),
    CurrentRow,
}

pub enum FrameBetween {
    UnboundedPreceeding,
    Preceeding(Expression),
    CurrentRow,
    Following(Expression),
}

pub enum FrameExclude {
    None,
    NoOthers,
    CurrentRow,
    Group,
    Ties,
}

pub enum LimitType {
    Normal(Expression, Expression),
    Offset(Expression, Expression),
}

pub enum CompoundOperator {
    Union(SelectClause),
    UnionAll(SelectClause),
    Intersect(SelectClause),
    Except(SelectClause),
}
