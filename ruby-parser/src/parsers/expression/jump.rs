use crate::lexer::*;
use crate::parsers::expression::argument::argument_list;

/// `return`
pub(crate) fn return_without_argument(i: Input) -> NodeResult {
    map(tag("return"), |_| Node::Return(vec![]))(i)
}

/// `return` *jump_argument*
pub(crate) fn return_with_argument(i: Input) -> NodeResult {
    map(preceded(tag("return"), jump_argument), |vec| {
        Node::Return(vec)
    })(i)
}

/// [ no ⏎ ] *argument_list*
pub(crate) fn jump_argument(i: Input) -> NodeListResult {
    preceded(no_lt, argument_list)(i)
}

/// `break`
pub(crate) fn break_without_argument(i: Input) -> NodeResult {
    map(tag("break"), |_| Node::Break(vec![]))(i)
}

/// `break` *jump_argument*
pub(crate) fn break_with_argument(i: Input) -> NodeResult {
    map(preceded(tag("break"), jump_argument), |vec| {
        Node::Break(vec)
    })(i)
}

/// `next`
pub(crate) fn next_without_argument(i: Input) -> NodeResult {
    map(tag("next"), |_| Node::Next(vec![]))(i)
}

/// `next` *jump_argument*
pub(crate) fn next_with_argument(i: Input) -> NodeResult {
    map(preceded(tag("next"), jump_argument), |vec| Node::Next(vec))(i)
}

/// `redo`
pub(crate) fn redo_expression(i: Input) -> NodeResult {
    map(tag("redo"), |_| Node::Redo)(i)
}

/// `retry`
pub(crate) fn retry_expression(i: Input) -> NodeResult {
    map(tag("retry"), |_| Node::Retry)(i)
}
