pub mod clauses;
pub mod expression;
pub mod functions;
pub mod literals;
pub mod statements;

use pest::{Parser, iterators::Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct SqlParser;

pub fn parse(sql_program: &str) -> Pairs<Rule> {
    SqlParser::parse(Rule::sql_program, sql_program).unwrap()
}
