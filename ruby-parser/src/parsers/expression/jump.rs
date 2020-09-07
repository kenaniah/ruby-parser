use crate::lexer::*;
use crate::parsers::expression::argument::argument_list;

/// `return`
pub(crate) fn return_without_argument(i: Input) -> NodeResult {
    map(tag("return"), |_| Node::Placeholder)(i)
}

/// `return` *jump_argument*
pub(crate) fn return_with_argument(i: Input) -> NodeResult {
    map(tuple((tag("return"), jump_argument)), |_| Node::Placeholder)(i)
}

/// [ no line terminator here ] *argument_list*
pub(crate) fn jump_argument(i: Input) -> NodeResult {
    map(tuple((no_lt, argument_list)), |_| Node::Placeholder)(i)
}

/// `break`
pub(crate) fn break_without_argument(i: Input) -> NodeResult {
    map(tag("break"), |_| Node::Placeholder)(i)
}

/// `break` *jump_argument*
pub(crate) fn break_with_argument(i: Input) -> NodeResult {
    map(tuple((tag("break"), jump_argument)), |_| Node::Placeholder)(i)
}

/// `next`
pub(crate) fn next_without_argument(i: Input) -> NodeResult {
    map(tag("next"), |_| Node::Placeholder)(i)
}

/// `next` *jump_argument*
pub(crate) fn next_with_argument(i: Input) -> NodeResult {
    map(tuple((tag("next"), jump_argument)), |_| Node::Placeholder)(i)
}

/// `redo`
pub(crate) fn redo_expression(i: Input) -> NodeResult {
    map(tag("redo"), |_| Node::Placeholder)(i)
}

/// `retry`
pub(crate) fn retry_expression(i: Input) -> NodeResult {
    map(tag("retry"), |_| Node::Placeholder)(i)
}
