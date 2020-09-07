use crate::lexer::*;
use crate::parsers::expression::argument::argument_list;
use crate::parsers::expression::argument::argument_without_parenthesis;

/// *yield_with_parenthesis_and_argument* | *yield_with_parenthesis_without_argument* | `yield`
pub(crate) fn yield_with_optional_argument(i: Input) -> NodeResult {
    alt((
        yield_with_parenthesis_and_argument,
        yield_with_parenthesis_without_argument,
        map(tag("yield"), |_| Node::Placeholder),
    ))(i)
}

/// `yield` [ no ⏎ ] [ no ⎵ ] `(` *argument_list `)`
pub(crate) fn yield_with_parenthesis_and_argument(i: Input) -> NodeResult {
    map(
        tuple((tag("yield("), ws, argument_list, ws, char(')'))),
        |_| Node::Placeholder,
    )(i)
}

/// `yield` [ no ⏎ ] [ no ⎵ ] `()`
pub(crate) fn yield_with_parenthesis_without_argument(i: Input) -> NodeResult {
    map(tuple((tag("yield("), ws, char(')'))), |_| Node::Placeholder)(i)
}

/// `yield` *argument_without_parenthesis*
pub(crate) fn yield_with_argument(i: Input) -> NodeResult {
    map(tuple((tag("yield"), argument_without_parenthesis)), |_| {
        Node::Placeholder
    })(i)
}
