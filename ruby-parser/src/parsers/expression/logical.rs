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
use nom::combinator::map;
use nom::sequence::tuple;

/// *keyword_not_expression* | *keyword_and_expression* | *keyword_or_expression*
pub(crate) fn keyword_logical_expression(i: Input) -> NodeResult {
    alt((
        keyword_not_expression,
        keyword_and_expression,
        keyword_or_expression,
    ))(i)
}

/// *method_invocation_without_parenthesis* | *operator_expression* | `!` *method_invocation_without_parenthesis* | `not` *keyword_not_expression*
pub(crate) fn keyword_not_expression(i: Input) -> NodeResult {
    alt((
        method_invocation_without_parenthesis,
        operator_expression,
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
    ))(i)
}

/// `!` ( *method_invocation_without_parenthesis* | *unary_expression* )
pub(crate) fn operator_not_expression(i: Input) -> NodeResult {
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

/// *equality_expression* | *operator_and_expression* [ no line terminator here ] `&&` *equality_expression*
pub(crate) fn operator_and_expression(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                operator_and_expression,
                no_lt,
                tag("&&"),
                ws,
                equality_expression,
            )),
            |t| {
                Node::LogicalAnd(LogicalAnd {
                    first: Box::new(t.0),
                    second: Box::new(t.4),
                })
            },
        ),
        equality_expression,
    ))(i)
}

/// *expression* [ no line terminator here ] `or` *keyword_not_expression*
pub(crate) fn keyword_or_expression(i: Input) -> NodeResult {
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
    alt((
        map(
            tuple((
                operator_or_expression,
                no_lt,
                tag("||"),
                ws,
                operator_and_expression,
            )),
            |t| {
                Node::LogicalOr(LogicalOr {
                    first: Box::new(t.0),
                    second: Box::new(t.4),
                })
            },
        ),
        operator_and_expression,
    ))(i)
}
