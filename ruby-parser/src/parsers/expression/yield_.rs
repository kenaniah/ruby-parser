use crate::lexer::*;

/// *yield_with_parenthesis_and_argument* | *yield_with_parenthesis_without_argument* | `yield`
pub(crate) fn yield_with_optional_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `yield` [ no line terminator here ] [ no whitespace here ] `(` *argument_list `)`
pub(crate) fn yield_with_parenthesis_and_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `yield` [ no line terminator here ] [ no whitespace here ] `()`
pub(crate) fn yield_with_parenthesis_without_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `yield` *argument_without_parenthesis*
pub(crate) fn yield_with_argument(i: Input) -> NodeResult {
    stub(i)
}
