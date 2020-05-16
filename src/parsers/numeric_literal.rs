use nom::character::complete::digit1;
use nom::character::complete::one_of;
use nom::branch::alt;
use nom::IResult;

use crate::token::Token;
//use crate::ParseResult;

pub fn numeric_literal(i: &str) -> IResult<&str, Token> {
    alt((signed_number, unsigned_number))(i)
}

pub fn signed_number(i: &str) -> IResult<&str, Token> {
    let (i, sign) = one_of("+-")(i)?;
    let (i, token) = unsigned_number(i)?;
    if sign == '-' {
        let token = match token {
            Token::Integer(v) => Token::Integer(v * -1),
            Token::Float(v) => Token::Float(v * -1f64),
            v @ _ => v
        };
        return Ok((i, token));
    }
    Ok((i, token))
}

pub fn unsigned_number(i: &str) -> IResult<&str, Token> {
    let (i, _) = digit1(i)?;
    Ok((i, Token::Integer(0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero() {
        assert_eq!(numeric_literal("0"), Ok(("", Token::Integer(0))));
        assert_eq!(
            numeric_literal("foobar"),
            Err(nom::Err::Error(("foobar", nom::error::ErrorKind::Digit)))
        )
    }
}
