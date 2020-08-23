use crate::parsers::expression::primary_expression;
use crate::*;
use nom::branch::alt;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::sequence::tuple;

/// *power_expression* | `-` *power_expression*
pub(crate) fn unary_minus_expression(i: Input) -> AstResult {
    stub(i)
}

/// *primary_expression* | `~` *unary_expression* | `+` *unary_expression* | `!` *unary_expression*
pub(crate) fn unary_expression(i: Input) -> AstResult {
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

fn stub(i: Input) -> AstResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
