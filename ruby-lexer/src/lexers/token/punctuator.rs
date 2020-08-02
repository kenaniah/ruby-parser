use crate::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::recognize;

pub fn punctuator(i: Input) -> TokenResult {
    let (i, res) = alt((
        tag("..."),
        tag(".."),
        tag("::"),
        tag("=>"),
        recognize(one_of("[](){},;:?")),
    ))(i)?;
    let token = match *res {
        "[" => Token::LeftBracket,
        "]" => Token::RightBracket,
        "(" => Token::LeftParen,
        ")" => Token::RightParen,
        "," => Token::Comma,
        ";" => Token::Semicolon,
        ":" => Token::Colon,
        "?" => Token::QuestionMark,
        ".." => Token::DotDot,
        "..." => Token::DotDotDot,
        "=>" => Token::Arrow,
        "::" => Token::DoubleColon,
        _ => unreachable!(),
    };
    Ok((i, token))
}
