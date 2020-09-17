use crate::lexer::*;
use crate::parsers::expression::association;
use crate::parsers::expression::method::chained_command_with_do_block;
use crate::parsers::expression::method::command;
use crate::parsers::expression::object::association_list;
use crate::parsers::expression::operator_expression;

/// *command* | *operator_expression_list* ( [ no ⏎ ] `,` )? | *operator_expression_list* ( [ no ⏎ ] `,` *splatting_argument* ) | *association_list* ( [ no ⏎ ] `,` )? | *splatting_argument*
pub(crate) fn indexing_argument_list(i: Input) -> NodeListResult {
    alt((
        map(command, |v| vec![v]),
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

/// [ no ⏎ ] `,`
pub(crate) fn comma(i: Input) -> LexResult {
    recognize(tuple((no_lt, char(','))))(i)
}

/// `*` *operator_expression*
pub(crate) fn splatting_argument(i: Input) -> NodeResult {
    map(tuple((char('*'), ws, operator_expression)), |t| {
        Node::Splat(Box::new(t.2))
    })(i)
}

/// *operator_expression* ( [ no ⏎ ] `,` *operator_expression* **but not** *association* )*
pub(crate) fn operator_expression_list(i: Input) -> NodeListResult {
    map(
        tuple((
            operator_expression,
            many0(map(
                tuple((comma, ws, peek(not(association)), operator_expression)),
                |t| t.3,
            )),
        )),
        |(first, mut vec)| {
            vec.insert(0, first);
            vec
        },
    )(i)
}

/// `()` | `(` *argument_list* `)` | `(` *operator_expression_list* [ no ⏎ ] `,` *chained_command_with_do_block* `)` | `(` *chained_command_with_do_block* `)`
pub(crate) fn argument_with_parenthesis(i: Input) -> NodeListResult {
    alt((
        map(tuple((char('('), ws, char(')'))), |_| {
            vec![Node::Placeholder]
        }),
        map(tuple((char('('), ws, argument_list, ws, char(')'))), |_| {
            vec![Node::Placeholder]
        }),
        map(
            tuple((
                char('('),
                ws,
                operator_expression_list,
                comma,
                chained_command_with_do_block,
                ws,
                char(')'),
            )),
            |_| vec![Node::Placeholder],
        ),
        map(
            tuple((char('('), ws, chained_command_with_do_block, ws, char(')'))),
            |_| vec![Node::Placeholder],
        ),
    ))(i)
}

/// **not** `{` [ no ⏎ ] *argument_list*
pub(crate) fn argument_without_parenthesis(i: Input) -> NodeListResult {
    map(tuple((not(peek(char('{'))), no_lt, argument_list)), |t| t.2)(i)
}

/// *block_argument* | *splatting_argument* ( [ no ⏎ ] `,` *block_argument* )? | *operator_expression_list* [ no ⏎ ] `,` *association_list* ( [ no ⏎ ] `,` *splatting_argument* )? ( [ no ⏎ ] `,` *block_argument* )? | ( *operator_expression_list* | *association_list* ) ( [ no ⏎ ] `,` *splatting_argument* )? ( [ no ⏎ ] `,` *block_argument* )? | *command*
pub(crate) fn argument_list(i: Input) -> NodeListResult {
    alt((
        map(block_argument, |v| vec![v]),
        map(
            tuple((splatting_argument, opt(tuple((comma, ws, block_argument))))),
            |t| {
                let mut vec = vec![t.0];
                if let Some((_, _, block)) = t.1 {
                    vec.push(block);
                }
                vec
            },
        ),
        map(
            tuple((
                //FIXME: needs to backtrack in order for association_list to work.
                // May need to attempt matching associations before operator expressions
                operator_expression_list,
                comma,
                ws,
                association_list,
                opt(tuple((comma, ws, splatting_argument))),
                opt(tuple((comma, ws, block_argument))),
            )),
            |t| {
                let mut vec = t.0;
                vec.push(Node::hash(t.3));
                if let Some((_, _, splat)) = t.4 {
                    vec.push(splat);
                }
                if let Some((_, _, block)) = t.5 {
                    vec.push(block);
                }
                vec
            },
        ),
        map(
            tuple((
                alt((
                    operator_expression_list,
                    map(association_list, |v| vec![Node::hash(v)]),
                )),
                opt(tuple((comma, ws, splatting_argument))),
                opt(tuple((comma, ws, block_argument))),
            )),
            |t| {
                let mut vec = t.0;
                if let Some((_, _, splat)) = t.1 {
                    vec.push(splat);
                }
                if let Some((_, _, block)) = t.2 {
                    vec.push(block);
                }
                vec
            },
        ),
        map(command, |v| vec![v]),
    ))(i)
}

/// `&` *operator_expression*
pub(crate) fn block_argument(i: Input) -> NodeResult {
    map(tuple((char('&'), ws, operator_expression)), |t| {
        Node::BlockArg(Box::new(t.2))
    })(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOpKind;

    #[test]
    fn test_argument_list() {
        use_parser!(argument_list);
        // Parse errors
        assert_err!("&");
        // Success cases
        assert_ok!("& :foo", vec![Node::block_arg(Node::literal_symbol("foo"))]);
        assert_ok!("*1", vec![Node::splat(Node::int(1))]);
        assert_ok!(
            "*1 ,\n&2",
            vec![Node::splat(Node::int(1)), Node::block_arg(Node::int(2))]
        );
        assert_ok!("1, 2,\n3", vec![Node::int(1), Node::int(2), Node::int(3)]);
        assert_ok!(
            "1, foo: 2, 3 => 4",
            vec![
                Node::int(1),
                Node::hash(vec![
                    Node::literal_symbol("foo"),
                    Node::int(2),
                    Node::int(3),
                    Node::int(4)
                ])
            ]
        );
    }

    #[test]
    fn test_splatting_argument() {
        use_parser!(splatting_argument);
        // Parse errors
        assert_err!("*");
        // Success cases
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
