use winnow::{
    combinator::{alt, opt, preceded, seq},
    ModalResult, Parser,
};

use crate::eplite::parser::{
    expressions::{prs_expression, Expression},
    identifiers::prs_collation_name,
    keywords::{prs_asc, prs_by, prs_collate, prs_desc, prs_first, prs_last, prs_nulls, prs_order},
};

#[derive(Debug)]
pub struct OrderByClause {
    pub expression: Expression,
    pub collation_name: Option<String>,
    pub ascending: bool,
    pub nulls_order: NullOrder,
}

#[derive(Debug)]
pub enum NullOrder {
    Default,
    NullsFirst,
    NullsLast,
}

pub fn prs_order_by_clause<'s>(input: &mut &'s str) -> ModalResult<Option<OrderByClause>> {
    opt(preceded(seq!(prs_order, prs_by), prs_ordering_term)).parse_next(input)
}

pub fn prs_ordering_term<'s>(input: &mut &'s str) -> ModalResult<OrderByClause> {
    seq! {
            OrderByClause {
            expression: prs_expression,
            collation_name: prs_collation,
            ascending: prs_ordering_direction,
            nulls_order: prs_null_order
        }
    }
    .parse_next(input)
}

pub fn prs_collation<'s>(input: &mut &'s str) -> ModalResult<Option<String>> {
    opt(preceded(prs_collate, prs_collation_name)).parse_next(input)
}

pub fn prs_ordering_direction<'s>(input: &mut &'s str) -> ModalResult<bool> {
    match opt(alt((prs_asc, prs_desc))).parse_next(input).unwrap() {
        Some(value) => Ok(value.eq("asc")),
        None => Ok(false),
    }
}

pub fn prs_null_order<'s>(input: &mut &'s str) -> ModalResult<NullOrder> {
    match opt(preceded(prs_nulls, alt((prs_first, prs_last))))
        .parse_next(input)
        .unwrap()
    {
        Some(direction) => Ok(if direction.eq("first") {
            NullOrder::NullsFirst
        } else {
            NullOrder::NullsLast
        }),
        None => Ok(NullOrder::Default),
    }
}
