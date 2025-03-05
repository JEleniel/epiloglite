use winnow::{ModalResult, Parser};

pub fn prs_asterisk<'s>(input: &mut &'s str) -> ModalResult<char> {
    '*'.parse_next(input)
}
