use crate::lexer::*;

/// `super` ( [ no line terminator here ] [ no whitespace here ] *argument_with_parenthesis* )? *block*?
pub(crate) fn super_with_optional_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `super` *argument_without_parenthesis*
pub(crate) fn super_with_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `super` *argument_without_parenthesis* *do_block*
pub(crate) fn super_with_argument_and_do_block(i: Input) -> NodeResult {
    stub(i)
}
