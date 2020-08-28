use crate::ast::{UnaryOp, UnaryOpToken};
use crate::lexer::*;
use crate::parsers::expression::binary::power_expression;
use crate::parsers::expression::primary_expression;
use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::combinator::map;
use nom::sequence::tuple;

/// *power_expression* | `-` *power_expression*
pub(crate) fn unary_minus_expression(i: Input) -> NodeResult {
    alt((
        map(tuple((char('-'), power_expression)), |t| {
            Node::UnaryOp(UnaryOp {
                op: UnaryOpToken::from(t.0),
                rhs: Box::new(t.1),
            })
        }),
        power_expression,
    ))(i)
}

/// *primary_expression* | `~` *unary_expression* | `+` *unary_expression* | `!` *unary_expression*
pub(crate) fn unary_expression(i: Input) -> NodeResult {
    alt((
        map(tuple((one_of("~+!"), unary_expression)), |t| {
            Node::UnaryOp(UnaryOp {
                op: UnaryOpToken::from(t.0),
                rhs: Box::new(t.1),
            })
        }),
        map(primary_expression, |t| Node::from(t)),
    ))(i)
}
