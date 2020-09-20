use crate::lexer::*;
use crate::parsers::expression::assignment::multiple::left_hand_side;
use crate::parsers::expression::assignment::multiple::multiple_left_hand_side;
use crate::parsers::program::compound_statement;

/// *brace_block* | *do block*
pub(crate) fn block(i: Input) -> NodeResult {
    alt((brace_block, do_block))(i)
}

/// `{` *block_parameter*? *block_body* `}`
pub(crate) fn brace_block(i: Input) -> NodeResult {
    map(
        tuple((char('{'), opt(block_parameter), block_body, char('}'))),
        |_| Node::Placeholder,
    )(i)
}

/// `do` *block_parameter*? *block_body* `end`
pub(crate) fn do_block(i: Input) -> NodeResult {
    map(
        tuple((tag("do"), opt(block_parameter), block_body, tag("end"))),
        |_| Node::Placeholder,
    )(i)
}

/// `| |` | `||` | `|` *block_parameter_list* `|`
pub(crate) fn block_parameter(i: Input) -> NodeResult {
    alt((
        map(alt((tag("| |"), tag("||"))), |_| Node::Placeholder),
        map(tuple((char('|'), block_parameter_list, char('|'))), |t| {
            Node::Placeholder
        }),
    ))(i)
}

/// *left_hand_side* | *multiple_left_hand_side*
pub(crate) fn block_parameter_list(i: Input) -> NodeResult {
    alt((left_hand_side, multiple_left_hand_side))(i)
}

/// *compound_statement*
pub(crate) fn block_body(i: Input) -> NodeResult {
    compound_statement(i)
}
