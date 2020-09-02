/*!
# Logical Operators

## Operator Precedence

## Grammar Transformations

In some cases, grammar had to be transformed in order to remove left recursion. Most notably,
the keyword logical expressions portion of the grammar suffered from indirect left recursion, which
had to be substantially refactored in order to preserve operator precedence.

The keyword logical expression grammar...
```text
E -> N | A | O        # keyword_logical_expression
N -> n N | x | y | z  # keyword_not_expression
A -> E a N            # keyword_and_expression
O -> E o N            # keyword_or_expression
```
Will be factored into...
```text
E  -> N               # keyword_logical_expression
    | A
    | O
N  -> n N             # keyword_not_expression
    | x                 # operator_expression (terminal)
    | y                 # ! method_invocation_without_parenthesis (terminal)
    | z                 # method_invocation_without_parenthesis (terminal)
A  -> N A1            # keyword_and_expression
    | O A1
O  -> N O1            # keyword_or_expression
    | N A1 O1
A1 -> a N A2
A2 -> A1
    | ϵ
O1 -> o N O2
O2 -> A1 O1
    | O1
    | ϵ
```

*/

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
    let i = stack_frame!("keyword_logical_expression", i);
    alt((
        keyword_not_expression,
        keyword_and_expression,
        keyword_or_expression,
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
pub(crate) fn keyword_and_expression(i: Input) -> NodeResult {
    let i = stack_frame!("keyword_and_expression", i);
    map(
        tuple((expression, opt(_keyword_and_expression))),
        |(node, ast)| update_placeholder!(Node::LogicalAnd, first, node, ast),
    )(i)
}

fn _keyword_and_expression(i: Input) -> NodeResult {
    let i = stack_frame!("keyword_and_expression", i);
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
pub(crate) fn keyword_or_expression(i: Input) -> NodeResult {
    let i = stack_frame!("keyword_or_expression", i);
    map(
        tuple((expression, opt(_keyword_or_expression))),
        |(node, ast)| update_placeholder!(Node::LogicalOr, first, node, ast),
    )(i)
}

fn _keyword_or_expression(i: Input) -> NodeResult {
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
            update_placeholder!(Node::LogicalOr, first, node, t.4)
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
