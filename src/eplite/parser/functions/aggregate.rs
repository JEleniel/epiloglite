use winnow::{
    combinator::{alt, seq},
    token::literal,
    ModalResult, Parser,
};

use crate::eplite::parser::{
    clauses::{
        distinct::prs_distinct_clause,
        filter::{prs_filter_clause, FilterClause},
        orderby::{prs_order_by_clause, OrderByClause},
    },
    expressions::{prs_expression, Expression},
    symbols::prs_asterisk,
};

#[derive(Debug)]
pub struct NonaryAggregateFunction<'a> {
    name: &'a str,
    asterisk: char,
}

#[derive(Debug)]
pub struct UnaryAggregateFunction<'a> {
    name: &'a str,
    distinct: bool,
    expression: Expression,
    order_by: Option<OrderByClause>,
    filter: Option<FilterClause>,
}

#[derive(Debug)]
pub struct BinaryAggregateFunction<'a> {
    name: &'a str,
    distinct: bool,
    expression1: Expression,
    expression2: Expression,
    order_by: Option<OrderByClause>,
    filter: Option<FilterClause>,
}

pub fn prs_nonary_aggregate_function<'s>(
    input: &mut &'s str,
) -> ModalResult<NonaryAggregateFunction<'s>> {
    seq! {
        NonaryAggregateFunction {
        name: literal("count"),
        asterisk: prs_asterisk,
    }}
    .parse_next(input)
}

pub fn prs_unary_aggregate_function<'s>(
    input: &mut &'s str,
) -> ModalResult<UnaryAggregateFunction<'s>> {
    seq! {
        UnaryAggregateFunction {
        name: alt(("avg","count","min","max","sum","group_concat","total")),
        distinct: prs_distinct_clause,
        expression: prs_expression,
        order_by: prs_order_by_clause,
        filter: prs_filter_clause,
    }}
    .parse_next(input)
}

pub fn prs_binary_aggregate_function<'s>(
    input: &mut &'s str,
) -> ModalResult<BinaryAggregateFunction<'s>> {
    seq! {
        BinaryAggregateFunction {
        name: alt(("group_concat","string_agg")),
        distinct: prs_distinct_clause,
        expression1: prs_expression,
        expression2: prs_expression,
        order_by: prs_order_by_clause,
        filter: prs_filter_clause,
    }}
    .parse_next(input)
}
