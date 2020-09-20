use crate::ast::{UnaryOp, UnaryOpKind as Op};
use crate::lexer::*;
use crate::parsers::expression::binary::power_expression;
use crate::parsers::expression::primary_expression;

/// *power_expression* | `-` *power_expression*
pub(crate) fn unary_minus_expression(i: Input) -> NodeResult {
    let i = stack_frame!("unary_minus_expression", i);
    alt((
        power_expression,
        map(tuple((char('-'), ws0, power_expression)), |t| {
            Node::UnaryOp(UnaryOp {
                op: Op::from(t.0),
                rhs: Box::new(t.2),
            })
        }),
    ))(i)
}

/// *primary_expression* | `~` *unary_expression* | `+` *unary_expression* | `!` *unary_expression*
pub(crate) fn unary_expression(i: Input) -> NodeResult {
    let i = stack_frame!("unary_expression", i);
    alt((
        map(tuple((one_of("~+!"), ws0, unary_expression)), |t| {
            Node::UnaryOp(UnaryOp {
                op: Op::from(t.0),
                rhs: Box::new(t.2),
            })
        }),
        map(primary_expression, |t| Node::from(t)),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unary_minus_expression() {
        use_parser!(unary_minus_expression);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("---42");
        assert_err!("- - 42");
        // Success cases
        assert_ok!("nil", Node::Nil);
        assert_ok!("-nil", Node::unary_op(Op::Negative, Node::Nil));
        assert_ok!("-\n\n  nil", Node::unary_op(Op::Negative, Node::Nil));
        assert_ok!("-42", Node::int(-42));
        assert_ok!("- 42", Node::unary_op(Op::Negative, Node::int(42)));
        assert_ok!("--42", Node::unary_op(Op::Negative, Node::int(-42)));
        // Integration cases
        assert_ok!(
            "!foo",
            Node::unary_op(
                Op::LogicalNot,
                Node::ident("foo", IdentifierKind::LocalVariable)
            )
        );
    }

    #[test]
    fn test_unary_expression() {
        use_parser!(unary_expression);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("bar\n");
        assert_err!("('");
        assert_err!("((foo)");
        // Success cases
        assert_ok!("nil", Node::Nil);
        assert_ok!("+42", Node::unary_op(Op::Positive, Node::int(42)));
        assert_ok!(
            "!! meh",
            Node::unary_op(
                Op::LogicalNot,
                Node::unary_op(
                    Op::LogicalNot,
                    Node::ident("meh", IdentifierKind::LocalVariable)
                )
            )
        );
        assert_ok!("-23e4", Node::float(-230000.0));
        assert_ok!("~(;)", Node::unary_op(Op::BitNot, Node::Block(vec![])));
    }
}
