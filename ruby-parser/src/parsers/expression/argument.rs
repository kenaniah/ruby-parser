use crate::lexer::*;
use crate::parsers::expression::method::command;
use crate::parsers::expression::object::association_list;
use crate::parsers::expression::operator_expression;
use crate::parsers::program::no_lt;
use crate::parsers::program::ws;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::{opt, recognize};
use nom::multi::many0;
use nom::sequence::tuple;

/// *command* | *operator_expression_list* ( [ no line terminator here ] `,` )? | *operator_expression_list* ( [ no line terminator here ] `,` *splatting_argument* ) | *association_list* ( [ no line terminator here ] `,` )? | *splatting_argument*
pub(crate) fn indexing_argument_list(i: Input) -> NodeResult {
    stub(i)
    // alt((
    //     command,
    //     tuple((operator_expression_list, opt(comma))),
    //     tuple((operator_expression_list, comma, splatting_argument)),
    //     tuple((association_list, opt(comma))),
    //     splatting_argument,
    // ))(i)
}

fn comma(i: Input) -> LexResult {
    recognize(tuple((no_lt, char(','))))(i)
}

/// `*` *operator_expression*
pub(crate) fn splatting_argument(i: Input) -> NodeResult {
    stub(i)
}

/// *operator_expression* ( [ no line terminator here ] `,` *operator_expression* )*
pub(crate) fn operator_expression_list(i: Input) -> NodeListResult {
    map(
        tuple((
            operator_expression,
            many0(map(
                tuple((no_lt, char(','), ws, operator_expression)),
                |t| t.3,
            )),
        )),
        |(first, mut vec)| {
            vec.insert(0, first);
            vec
        },
    )(i)
}

/// `()` | `(` *argument_list* `)` | `(` *operator_expression_list* [ no line terminator here ] `,` *chained_command_with_do_block* `)` | `(` *chained_command_with_do_block* `)`
pub(crate) fn argument_with_parentheses(i: Input) -> NodeResult {
    stub(i)
}

/// *block_argument* | *splatting_argument* ( `,` *block_argument* )? | *operator_expression_list* [ no line terminator here ] `,` *association_list* ( [ no line terminator here ] `,` *splatting_argument* )? ( [ no line terminator here ] `,` *block_argument* )? | ( *operator_expression_list* | *association_list* ) ( [ no line terminator here ] `,` *splatting_argument* )? ( [no line terminator here ] `,` *block_argument* )? | *command*
pub(crate) fn argument_list(i: Input) -> NodeListResult {
    stub_list(i)
}

/// `&` *operator_expression*
pub(crate) fn block_argument(i: Input) -> NodeResult {
    stub(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_expression_list() {
        use_parser!(operator_expression_list);
        // Parse errors
        assert_err!("");
        assert_err!("if");
        assert_err!("1, 2\n, 3");
        // Success cases
        assert_ok!("hi", vec![Node::ident("hi", IdentifierKind::LocalVariable)]);
        assert_ok!(
            "1, 2,\n3",
            vec![Node::integer(1), Node::integer(2), Node::integer(3)]
        );
    }
}
