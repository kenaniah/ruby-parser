use crate::lexer::*;
use crate::parsers::expression::method::command;
use crate::parsers::expression::object::association_list;
use crate::parsers::expression::operator_expression;

/// *command* | *operator_expression_list* ( [ no line terminator here ] `,` )? | *operator_expression_list* ( [ no line terminator here ] `,` *splatting_argument* ) | *association_list* ( [ no line terminator here ] `,` )? | *splatting_argument*
pub(crate) fn indexing_argument_list(i: Input) -> NodeListResult {
    alt((
        command,
        terminated(operator_expression_list, opt(comma)),
        map(
            tuple((operator_expression_list, comma, splatting_argument)),
            |mut t| {
                t.0.push(t.2);
                t.0
            },
        ),
        terminated(association_list, opt(comma)),
        map(splatting_argument, |v| vec![v]),
    ))(i)
}

/// [ no line terminator here ] `,`
pub(crate) fn comma(i: Input) -> LexResult {
    recognize(tuple((no_lt, char(','))))(i)
}

/// `*` *operator_expression*
pub(crate) fn splatting_argument(i: Input) -> NodeResult {
    map(tuple((char('*'), operator_expression)), |t| {
        Node::Splat(Box::new(t.1))
    })(i)
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
pub(crate) fn argument_with_parenthesis(i: Input) -> NodeListResult {
    stub_list(i)
}

/// **not** `{` [ no line terminator here ] *argument_list*
pub(crate) fn argument_without_parenthesis(i: Input) -> NodeListResult {
    map(tuple((not(peek(char('{'))), no_lt, argument_list)), |t| t.2)(i)
}

/// *block_argument* | *splatting_argument* ( `,` *block_argument* )? | *operator_expression_list* [ no line terminator here ] `,` *association_list* ( [ no line terminator here ] `,` *splatting_argument* )? ( [ no line terminator here ] `,` *block_argument* )? | ( *operator_expression_list* | *association_list* ) ( [ no line terminator here ] `,` *splatting_argument* )? ( [no line terminator here ] `,` *block_argument* )? | *command*
pub(crate) fn argument_list(i: Input) -> NodeListResult {
    stub_list(i)
}

/// `&` *operator_expression*
pub(crate) fn block_argument(i: Input) -> NodeResult {
    map(tuple((char('&'), operator_expression)), |t| {
        Node::BlockArg(Box::new(t.1))
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOpKind;

    #[test]
    fn test_splatting_argument() {
        use_parser!(splatting_argument);
        // Parse errors
        assert_err!("*");
        assert_ok!("*3", Node::splat(Node::int(3)));
    }

    #[test]
    fn test_operator_expression_list() {
        use_parser!(operator_expression_list);
        // Parse errors
        assert_err!("");
        assert_err!("if");
        assert_err!("1, 2\n, 3");
        // Success cases
        assert_ok!("hi", vec![Node::ident("hi", IdentifierKind::LocalVariable)]);
        assert_ok!("1, 2,\n3", vec![Node::int(1), Node::int(2), Node::int(3)]);
    }

    #[test]
    fn test_block_argument() {
        use_parser!(block_argument);
        // Parse errors
        assert_err!("&");
        assert_ok!(
            "&:foo - 2",
            Node::block_arg(Node::binary_op(
                Node::literal_symbol("foo"),
                BinaryOpKind::Subtract,
                Node::int(2)
            ))
        );
    }
}
