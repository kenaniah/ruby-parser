use crate::lexer::*;
use crate::parsers::expression::{expression, operator_expression};
use crate::parsers::program::ws;
use nom::bytes::complete::tag;
use nom::character::streaming::char;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::peek;
use nom::sequence::tuple;

/// `defined?` `(` *expression* `)`
pub(crate) fn defined_with_parenthesis(i: Input) -> NodeResult {
    map(
        tuple((tag("defined?"), char('('), ws, expression, ws, char(')'))),
        |t| Node::Defined(Box::new(t.3)),
    )(i)
}

/// `defined?` *operator_expression*
pub(crate) fn defined_without_parenthesis(i: Input) -> NodeResult {
    map(
        tuple((
            tag("defined?"),
            peek(not(char('('))),
            ws,
            operator_expression,
        )),
        |t| Node::Defined(Box::new(t.3)),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOpKind;

    #[test]
    fn test_defined_with_parenthesis() {
        use_parser!(defined_with_parenthesis);
        // Parse errors
        assert_err!("defined?");
        assert_err!("defined?()");
        // Success cases
        assert_ok!(
            "defined?( foo )",
            Node::Defined(Box::new(Node::ident("foo", IdentifierKind::LocalVariable)))
        );
        assert_ok!(
            "defined?(2 + 1)",
            Node::Defined(Box::new(Node::binary_op(
                Node::integer(2),
                BinaryOpKind::Add,
                Node::integer(1)
            )))
        );
    }

    #[test]
    fn test_defined_without_parenthesis() {
        use_parser!(defined_without_parenthesis);
        // Parse errors
        assert_err!("defined?");
        assert_err!("defined?()");
        // Success cases
        assert_ok!("defined? ()", Node::defined(Node::empty()));
        assert_ok!(
            "defined? foo",
            Node::defined(Node::ident("foo", IdentifierKind::LocalVariable))
        );
        assert_ok!(
            "defined? foo",
            Node::defined(Node::ident("foo", IdentifierKind::LocalVariable))
        );
        assert_ok!(
            "defined? 2 + 1",
            Node::defined(Node::binary_op(
                Node::integer(2),
                BinaryOpKind::Add,
                Node::integer(1)
            ))
        );
        assert_ok!(
            "defined?\n(2 +\n 1)",
            Node::defined(Node::Block(vec![Node::binary_op(
                Node::integer(2),
                BinaryOpKind::Add,
                Node::integer(1)
            )]))
        );
    }
}
