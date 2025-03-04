use winnow::{
    ascii::space1,
    combinator::{alt, seq},
    error::ContextError,
    ModalResult, Parser,
};

use super::keywords::{keyword_parser_create, keyword_parser_temp, keyword_parser_temporary};

pub fn create_view_parser<'s>(input: &mut &'s str) -> ModalResult<&'s str, ContextError> {
    seq! {
        keyword_parser_create(input),
    }
    .parse_next(input)
}
