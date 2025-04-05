use super::{
    clauses::CastClause,
    functions::{
        general_functions::Function, matching_functions::MatchFunction, operators::Operator,
    },
    literals::{Level3Name, Literal},
    statements::select::SelectStatment,
};

pub enum Expression {
    Literal(Literal),
    Bind(BindParameter),
    Column(Level3Name),
    Operator(Operator),
    Function(Function),
    Expressions(Vec<Box<Expression>>),
    Cast(CastClause),
    Match(MatchFunction),
    Exists(Box<SelectStatment>),
    Raise(RaiseFunction),
}

pub struct BindParameter {
    pub index: u64,
    pub name: String,
}

pub enum RaiseFunction {
    Ignore,
    Rollback(Box<Expression>),
    Abort(Box<Expression>),
    Fail(Box<Expression>),
}
