use winnow::{combinator::opt, ModalResult, Parser};

use crate::eplite::parser::keywords::prs_distinct;

pub fn prs_distinct_clause<'s>(input: &mut &'s str) -> ModalResult<bool> {
    Ok(match opt(prs_distinct).parse_next(input).unwrap() {
        Some(_) => true,
        None => false,
    })
}
