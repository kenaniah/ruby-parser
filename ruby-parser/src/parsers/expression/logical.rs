use crate::ast::{LogicalAnd, LogicalNot, LogicalOr};
use crate::lexer::*;
use crate::parsers::expression::binary::equality_expression;
use crate::parsers::expression::expression;
use crate::parsers::expression::method::method_invocation_without_parenthesis;
use crate::parsers::expression::operator_expression;
use crate::parsers::expression::unary::unary_expression;
use crate::parsers::program::{no_lt, ws};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::sequence::tuple;

/// *keyword_not_expression* | *keyword_and_expression* | *keyword_or_expression*
pub(crate) fn keyword_logical_expression(i: Input) -> NodeResult {
    println!("In keyword_logical_expression {}", i);
    alt((
        keyword_not_expression,
        keyword_and_expression,
        keyword_or_expression,
    ))(i)
}

/// *method_invocation_without_parenthesis* | *operator_expression* | `!` *method_invocation_without_parenthesis* | `not` *keyword_not_expression*
pub(crate) fn keyword_not_expression(i: Input) -> NodeResult {
    println!("In keyword_not_expression {}", i);
    alt((
        map(
            tuple((char('!'), ws, method_invocation_without_parenthesis)),
            |t| {
                Node::LogicalNot(LogicalNot {
                    expr: Box::new(t.2),
                })
            },
        ),
        map(tuple((tag("not"), ws, keyword_not_expression)), |t| {
            Node::LogicalNot(LogicalNot {
                expr: Box::new(t.2),
            })
        }),
        operator_expression,
        method_invocation_without_parenthesis,
    ))(i)
}

/// `!` ( *method_invocation_without_parenthesis* | *unary_expression* )
pub(crate) fn operator_not_expression(i: Input) -> NodeResult {
    println!("In operator_not_expression {}", i);
    map(
        tuple((
            char('!'),
            ws,
            alt((method_invocation_without_parenthesis, unary_expression)),
        )),
        |t| {
            Node::LogicalNot(LogicalNot {
                expr: Box::new(t.2),
            })
        },
    )(i)
}

/// *expression* [ no line terminator here ] `and` *keyword_not_expression*
pub(crate) fn keyword_and_expression(i: Input) -> NodeResult {
    println!("In keyword_and_expression {}", i);
    map(
        tuple((expression, no_lt, tag("and"), ws, keyword_not_expression)),
        |t| {
            Node::LogicalAnd(LogicalAnd {
                first: Box::new(t.0),
                second: Box::new(t.4),
            })
        },
    )(i)
}

/// *expression* [ no line terminator here ] `or` *keyword_not_expression*
pub(crate) fn keyword_or_expression(i: Input) -> NodeResult {
    println!("In keyword_or_expression {}", i);
    map(
        tuple((expression, no_lt, tag("or"), ws, keyword_not_expression)),
        |t| {
            Node::LogicalOr(LogicalOr {
                first: Box::new(t.0),
                second: Box::new(t.4),
            })
        },
    )(i)
}

/// *operator_and_expression* | *operator_or_expression* [ no line terminator here ] `||` *operator_and_expression*
pub(crate) fn operator_or_expression(i: Input) -> NodeResult {
    println!("In operator_or_expression {}", i);
    map(
        tuple((operator_and_expression, opt(_operator_or_expression))),
        |(lhs, ast)| match ast {
            Some(node @ Node::LogicalOr(_)) => _replace_nested_or_placeholder(node, lhs),
            _ => lhs,
        },
    )(i)
}

fn _operator_or_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                no_lt,
                tag("||"),
                ws,
                operator_and_expression,
                opt(_operator_or_expression),
            )),
            |t| {
                let node = Node::LogicalOr(LogicalOr {
                    first: Box::new(Node::Placeholder),
                    second: Box::new(t.3),
                });
                if let Some(parent_node) = t.4 {
                    _replace_nested_or_placeholder(parent_node, node)
                } else {
                    node
                }
            },
        ),
        operator_and_expression,
    ))(i)
}

/// *equality_expression* | *operator_and_expression* [ no line terminator here ] `&&` *equality_expression*
pub(crate) fn operator_and_expression(i: Input) -> NodeResult {
    println!("In operator_and_expression {}", i);
    map(
        tuple((equality_expression, opt(_operator_and_expression))),
        |(lhs, ast)| match ast {
            Some(node @ Node::LogicalAnd(_)) => _replace_nested_and_placeholder(node, lhs),
            _ => lhs,
        },
    )(i)
}

fn _operator_and_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                no_lt,
                tag("&&"),
                ws,
                equality_expression,
                opt(_operator_and_expression),
            )),
            |t| {
                let node = Node::LogicalAnd(LogicalAnd {
                    first: Box::new(Node::Placeholder),
                    second: Box::new(t.3),
                });
                if let Some(parent_node) = t.4 {
                    _replace_nested_and_placeholder(parent_node, node)
                } else {
                    node
                }
            },
        ),
        equality_expression,
    ))(i)
}

/// Recursively travels nested LogicalAnd nodes and replaces the last lhs with the given value
fn _replace_nested_and_placeholder(mut node: Node, value: Node) -> Node {
    use std::borrow::BorrowMut;
    {
        let mut n = &mut node;
        while let Node::LogicalAnd(sub) = n {
            n = sub.first.borrow_mut();
        }
        *n = value;
    }
    node
}

/// Recursively travels nested LogicalOr nodes and replaces the last lhs with the given value
fn _replace_nested_or_placeholder(mut node: Node, value: Node) -> Node {
    use std::borrow::BorrowMut;
    {
        let mut n = &mut node;
        while let Node::LogicalOr(sub) = n {
            n = sub.first.borrow_mut();
        }
        *n = value;
    }
    node
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOpKind;

    #[test]
    fn test_operator_or_expression() {
        use_parser!(operator_or_expression);
        // Parse errors
        assert_err!("");
        assert_err!("||");
        // Success cases
        assert_ok!(
            "1 || 2 && 3",
            Node::logical_or(
                Node::integer(1),
                Node::logical_and(Node::integer(2), Node::integer(3)),
            )
        );
        assert_ok!(
            "1 || 2 || 3",
            Node::logical_or(
                Node::logical_or(Node::integer(1), Node::integer(2)),
                Node::integer(3),
            )
        );
        assert_ok!(
            "1 && 2 || 3",
            Node::logical_or(
                Node::logical_and(Node::integer(1), Node::integer(2)),
                Node::integer(3),
            )
        );
    }

    #[test]
    fn test_operator_and_expression() {
        use_parser!(operator_and_expression);
        // Parse errors
        assert_err!("");
        assert_err!("2 &&");
        // Success cases
        assert_ok!(
            "1 && 2 != 3",
            Node::logical_and(
                Node::integer(1),
                Node::binary_op(Node::integer(2), BinaryOpKind::NotEqual, Node::integer(3)),
            )
        );
        assert_ok!(
            "1 && 2 && 3",
            Node::logical_and(
                Node::logical_and(Node::integer(1), Node::integer(2)),
                Node::integer(3),
            )
        );
    }
}
