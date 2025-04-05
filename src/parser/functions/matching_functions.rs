use crate::parser::expression::Expression;

pub struct MatchFunction {
    pub function_name: MatchFunctionName,
    pub negated: bool,
    pub expr1: Box<Expression>,
    pub expr2: Box<Expression>,
}

pub enum MatchFunctionName {
    Like(Option<Box<Expression>>),
    Glob,
    RegExp,
    Match,
}
