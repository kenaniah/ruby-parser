/*!
# Logical Operators

## Operator Precedence

## Grammar Transformations

In some cases, grammar had to be transformed in order to remove left recursion. Most notably,
the keyword logical expressions portion of the grammar suffered from indirect left recursion, which
had to be substantially refactored in order to preserve operator precedence.

The keyword logical expression grammar...
```text
E -> N | A | O          # keyword_logical_expression
N -> n N | x | y | z    # keyword_not_expression    n = "not"
A -> E a N              # keyword_and_expression    a = "and"
O -> E o N              # keyword_or_expression     o = "or"
```
Will be factored into...
```text
E  -> N | A | O         # keyword_logical_expression

N  -> n N | x | y | z   # keyword_not_expression    n = "not"

A  -> N A1 | O A1       # keyword_and_expression
A1 -> a N A1 | ϵ                                    a = "and"

O  -> N O1 | N A1 O1    # keyword_or_expression
O1 -> A1 O1 | o N O1 | ϵ                            o = "or"
```
*/

use crate::ast::{LogicalAnd, LogicalNot, LogicalOr};
use crate::lexer::*;
use crate::parsers::expression::binary::equality_expression;
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
    let i = stack_frame!("keyword_logical_expression", i);
    alt((
        keyword_or_expression,
        keyword_and_expression,
        keyword_not_expression,
    ))(i)
}

/// *method_invocation_without_parenthesis* | *operator_expression* | `!` *method_invocation_without_parenthesis* | `not` *keyword_not_expression*
pub(crate) fn keyword_not_expression(i: Input) -> NodeResult {
    let i = stack_frame!("keyword_not_expression", i);
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
    let i = stack_frame!("operator_not_expression", i);
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
/// `A  -> N A1 | O A1`
pub(crate) fn keyword_and_expression(i: Input) -> NodeResult {
    let i = stack_frame!("keyword_and_expression", i);
    map(
        tuple((
            alt((keyword_not_expression, keyword_or_expression)),
            _keyword_and_expression,
        )),
        |(node, ast)| {
            println!("{:?} - {:?}", node, ast);
            update_placeholder!(Node::LogicalAnd, first, node, Some(ast))
        },
    )(i)
}

/// `A1 -> a N A1 | ϵ`
fn _keyword_and_expression(i: Input) -> NodeResult {
    map(
        tuple((
            no_lt,
            tag("and"),
            ws,
            keyword_not_expression,
            opt(_keyword_and_expression),
        )),
        |t| {
            let node = Node::LogicalAnd(LogicalAnd {
                first: Box::new(Node::Placeholder),
                second: Box::new(t.3),
            });
            update_placeholder!(Node::LogicalAnd, first, node, t.4)
        },
    )(i)
}

/// *expression* [ no line terminator here ] `or` *keyword_not_expression*
/// `O  -> N O1 | N A1 O1`
pub(crate) fn keyword_or_expression(i: Input) -> NodeResult {
    let i = stack_frame!("keyword_or_expression", i);
    alt((
        map(
            tuple((keyword_not_expression, _keyword_or_expression)),
            |(node, ast)| {
                println!("1: {:?} - {:?}", node, ast);
                let res = update_placeholder!(Node::LogicalOr, first, node, Some(ast));
                println!("1 -> {:?}", res);
                res
            },
        ),
        map(
            tuple((
                keyword_not_expression,
                keyword_and_expression,
                _keyword_or_expression,
            )),
            |(node, mid, ast)| {
                println!("2: {:?} - {:?} - {:?}", node, mid, ast);
                let mid = update_placeholder!(Node::LogicalAnd, first, node, Some(mid));
                let res = update_placeholder!(Node::LogicalOr, first, mid, Some(ast));
                println!("2 -> {:?}", res);
                res
            },
        ),
    ))(i)
}

/// `O1 -> A1 O1 | o N O1 | ϵ`
fn _keyword_or_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((_keyword_and_expression, opt(_keyword_or_expression))),
            |(node, ast)| {
                println!("3: {:?} - {:?}", node, ast);
                let res = update_placeholder!(Node::LogicalAnd, first, node, ast);
                println!("3 -> {:?}", res);
                res
            },
        ),
        map(
            tuple((
                no_lt,
                tag("or"),
                ws,
                keyword_not_expression,
                opt(_keyword_or_expression),
            )),
            |t| {
                let node = Node::LogicalOr(LogicalOr {
                    first: Box::new(Node::Placeholder),
                    second: Box::new(t.3),
                });
                println!("4: {:?} - {:?}", node, t.4);
                let res = update_placeholder!(Node::LogicalAnd, first, node, t.4);
                println!("4 -> {:?}", res);
                res
            },
        ),
    ))(i)
}

fn __keyword_or_expression(i: Input) -> NodeResult {
    map(
        tuple((opt(_keyword_and_expression), _keyword_or_expression)),
        |(node, ast)| {
            if let Some(node) = node {
                update_placeholder!(Node::LogicalOr, first, node, Some(ast))
            } else {
                ast
            }
        },
    )(i)
}

/// *operator_and_expression* | *operator_or_expression* [ no line terminator here ] `||` *operator_and_expression*
pub(crate) fn operator_or_expression(i: Input) -> NodeResult {
    let i = stack_frame!("operator_or_expression", i);
    map(
        tuple((operator_and_expression, opt(_operator_or_expression))),
        |(node, ast)| update_placeholder!(Node::LogicalOr, first, node, ast),
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
                update_placeholder!(Node::LogicalOr, first, node, t.4)
            },
        ),
        operator_and_expression,
    ))(i)
}

/// *equality_expression* | *operator_and_expression* [ no line terminator here ] `&&` *equality_expression*
pub(crate) fn operator_and_expression(i: Input) -> NodeResult {
    let i = stack_frame!("operator_and_expression", i);
    map(
        tuple((equality_expression, opt(_operator_and_expression))),
        |(node, ast)| update_placeholder!(Node::LogicalAnd, first, node, ast),
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
                update_placeholder!(Node::LogicalAnd, first, node, t.4)
            },
        ),
        equality_expression,
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOpKind;

    #[test]
    fn test_keyword_logical_expression() {
        use_parser!(keyword_logical_expression);
        // Parse errors
        // assert_err!("or");
        // assert_err!("and not");
        // Success cases
        assert_ok!(
            "1 or 2 and 3",
            Node::logical_and(
                Node::logical_or(Node::integer(1), Node::integer(2)),
                Node::integer(3)
            )
        );
        // assert_ok!(
        //     "1 and 2 or 3",
        //     Node::logical_or(
        //         Node::logical_and(Node::integer(1), Node::integer(2)),
        //         Node::integer(3)
        //     )
        // );
        // assert_ok!(
        //     "1 and 2 and not 3 or 4 or 5 and 6 and 7" // "1 and 2 or 3",
        //                                               // Node::logical_or(
        //                                               //     Node::logical_and(Node::integer(1), Node::integer(2)),
        //                                               //     Node::integer(3)
        //                                               // )
        // );
    }

    #[test]
    fn test_keyword_or_expression() {
        use_parser!(keyword_or_expression);
        // Parse errors
        assert_err!("or");
        assert_err!("1 or");
        // Success cases
        assert_ok!(
            "1 or 2",
            Node::logical_or(Node::integer(1), Node::integer(2))
        );
        assert_ok!(
            "not 1 or not 2",
            Node::logical_or(
                Node::logical_not(Node::integer(1)),
                Node::logical_not(Node::integer(2))
            )
        );
        assert_ok!(
            "1 or 2 or 3",
            Node::logical_or(
                Node::logical_or(Node::integer(1), Node::integer(2)),
                Node::integer(3)
            )
        );
    }

    #[test]
    fn test_keyword_and_expression() {
        use_parser!(keyword_and_expression);
        // Parse errors
        assert_err!("and");
        assert_err!("1 and");
        // Success cases
        assert_ok!(
            "1 and 2",
            Node::logical_and(Node::integer(1), Node::integer(2))
        );
        assert_ok!(
            "not 1 and not 2",
            Node::logical_and(
                Node::logical_not(Node::integer(1)),
                Node::logical_not(Node::integer(2))
            )
        );
        assert_ok!(
            "1 and 2 and 3",
            Node::logical_and(
                Node::logical_and(Node::integer(1), Node::integer(2)),
                Node::integer(3)
            )
        );
    }

    #[test]
    fn test_keyword_not_expression() {
        use_parser!(keyword_not_expression);
        // Parse errors
        assert_err!("");
        assert_err!("not");
        assert_err!("not not");
        // Success cases
        assert_ok!("not true", Node::logical_not(Node::boolean(true)));
        assert_ok!(
            "not not false",
            Node::logical_not(Node::logical_not(Node::boolean(false)))
        );
    }

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
