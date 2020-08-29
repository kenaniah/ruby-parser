use crate::ast::{UnaryOp, UnaryOpToken};
use crate::lexer::*;
use crate::parsers::expression::binary::power_expression;
use crate::parsers::expression::primary_expression;
use crate::parsers::program::ws;
use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::combinator::map;
use nom::sequence::tuple;

/// *power_expression* | `-` *power_expression*
pub(crate) fn unary_minus_expression(i: Input) -> NodeResult {
    alt((
        power_expression,
        map(tuple((char('-'), ws, power_expression)), |t| {
            Node::UnaryOp(UnaryOp {
                op: UnaryOpToken::from(t.0),
                rhs: Box::new(t.2),
            })
        }),
    ))(i)
}

/// *primary_expression* | `~` *unary_expression* | `+` *unary_expression* | `!` *unary_expression*
pub(crate) fn unary_expression(i: Input) -> NodeResult {
    alt((
        map(tuple((one_of("~+!"), ws, unary_expression)), |t| {
            Node::unary_op(UnaryOpToken::from(t.0), t.2)
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
        assert_err!("---42"); // Handled at a higher level in the grammar
        assert_err!("- - 42"); // Handled at a higher level in the grammar
        // Success cases
        assert_ok!("nil", Node::Nil);
        assert_ok!("-nil", Node::unary_op(UnaryOpToken::Negative, Node::Nil));
        assert_ok!(
            "-\n\n  nil",
            Node::unary_op(UnaryOpToken::Negative, Node::Nil)
        );
        assert_ok!("-42", Node::integer(-42));
        assert_ok!(
            "- 42",
            Node::unary_op(UnaryOpToken::Negative, Node::integer(42))
        );
        assert_ok!(
            "--42",
            Node::unary_op(UnaryOpToken::Negative, Node::integer(-42))
        );
        // Integration cases
        assert_ok!(
            "!foo",
            Node::unary_op(
                UnaryOpToken::LogicalNot,
                Node::ident("foo", IdentifierType::LocalVariable)
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
        assert_ok!(
            "+42",
            Node::unary_op(UnaryOpToken::Positive, Node::integer(42))
        );
        assert_ok!(
            "!! meh",
            Node::unary_op(
                UnaryOpToken::LogicalNot,
                Node::unary_op(
                    UnaryOpToken::LogicalNot,
                    Node::ident("meh", IdentifierType::LocalVariable)
                )
            )
        );
        // assert_ok!("-23e4", Node::float(-230000.0));
        // assert_ok!("'hello world'", Node::literal_string("hello world"));
        assert_ok!(
            "~(;)",
            Node::unary_op(UnaryOpToken::BitNot, Node::Block(vec![]))
        );
        // assert_ok!(
        //     "((false))",
        //     Node::Block(vec![Node::Block(vec![Node::boolean(false)])])
        // );
        // assert_ok!(
        //     "(;2\n\t5;;)",
        //     Node::Block(vec![Node::integer(2), Node::integer(5)])
        // );
        // assert_ok!("(;)", Node::Block(vec![]));
    }
}
