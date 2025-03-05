use winnow::{
    combinator::{delimited, opt, preceded, seq},
    ModalResult, Parser,
};

use crate::eplite::parser::expressions::{prs_expression, Expression};
use crate::eplite::parser::keywords::{prs_filter, prs_where};

#[derive(Debug)]
pub struct FilterClause {
    expression: Expression,
}

pub fn prs_filter_clause<'s>(input: &mut &'s str) -> ModalResult<Option<FilterClause>> {
    match opt(preceded(
        prs_filter,
        delimited("(", preceded(prs_where, prs_expression), ")"),
    ))
    .parse_next(input)
    .unwrap()
    {
        Some(expression) => Ok(Some(FilterClause { expression })),
        None => Ok(None),
    }
}
