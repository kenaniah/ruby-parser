use crate::lexer::*;

/// `super` ( [ no line terminator here ] [ no whitespace here ] *argument_with_parenthesis* )? *block*?
pub(crate) fn super_with_optional_argument(i: Input) -> TokenResult {
    stub(i)
}

/// `super` *argument_without_parenthesis*
pub(crate) fn super_with_argument(i: Input) -> TokenResult {
    stub(i)
}

/// `super` *argument_without_parenthesis* *do_block*
pub(crate) fn super_with_argument_and_do_block(i: Input) -> TokenResult {
    stub(i)
}

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
