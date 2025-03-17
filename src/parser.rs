pub mod aggregate_functions;
pub mod date_time_functions;
pub mod expression;
pub mod literals;
pub mod operators;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/sql.pest"]
struct SqlParser;
