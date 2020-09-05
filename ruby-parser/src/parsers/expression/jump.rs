use crate::lexer::*;

/// `return`
pub(crate) fn return_without_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `return` *jump_argument*
pub(crate) fn return_with_argument(i: Input) -> NodeResult {
    stub(i)
}

/// [ no line terminator here ] *argument_list*
pub(crate) fn jump_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `break`
pub(crate) fn break_without_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `break` *jump_argument*
pub(crate) fn break_with_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `next`
pub(crate) fn next_without_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `next` *jump_argument*
pub(crate) fn next_with_argument(i: Input) -> NodeResult {
    stub(i)
}

/// `redo`
pub(crate) fn redo_expression(i: Input) -> NodeResult {
    stub(i)
}

/// `retry`
pub(crate) fn retry_expression(i: Input) -> NodeResult {
    stub(i)
}
