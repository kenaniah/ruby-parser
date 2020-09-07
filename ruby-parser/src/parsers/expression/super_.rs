use crate::lexer::*;
use crate::parsers::expression::argument::argument_with_parenthesis;
use crate::parsers::expression::argument::argument_without_parenthesis;
use crate::parsers::expression::block::block;
use crate::parsers::expression::block::do_block;

/// `super` ( [ no line terminator here ] [ no whitespace here ] *argument_with_parenthesis* )? *block*?
pub(crate) fn super_with_optional_argument(i: Input) -> NodeResult {
    map(
        tuple((tag("super"), opt(argument_with_parenthesis), opt(block))),
        |_| Node::Placeholder,
    )(i)
}

/// `super` *argument_without_parenthesis*
pub(crate) fn super_with_argument(i: Input) -> NodeResult {
    map(tuple((tag("super"), argument_without_parenthesis)), |_| {
        Node::Placeholder
    })(i)
}

/// `super` *argument_without_parenthesis* *do_block*
pub(crate) fn super_with_argument_and_do_block(i: Input) -> NodeResult {
    map(
        tuple((tag("super"), argument_without_parenthesis, do_block)),
        |_| Node::Placeholder,
    )(i)
}
