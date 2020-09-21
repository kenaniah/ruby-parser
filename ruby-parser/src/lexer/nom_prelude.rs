pub(crate) use crate::parsers::program::{no_lt, ws0};
pub(crate) use nom::branch::alt;
pub(crate) use nom::character::complete::{anychar, char, none_of, one_of};
pub(crate) use nom::combinator::{map, not, opt, peek, recognize, value, verify};
pub(crate) use nom::multi::{many0, many1, many_m_n, separated_list0, separated_list1};
pub(crate) use nom::sequence::{delimited, preceded, terminated, tuple};

use crate::lexer::{Input, LexResult};
use crate::parsers::token::identifier::identifier_character;
use nom::bytes::complete::tag as nom_tag;

/// Recognizes a pattern
///
/// The input data will be compared to the tag combinator's argument and will return the part of the input that matches the argument. Tags that start with an identifying character must not be followed by an identifying character in order for a match to be successful.
///
/// It will return `Err(Err::Error((_, ErrorKind::Tag)))` if the input doesn't match the pattern
///
/// See documentation for `nom::bytes::complete::tag` for more info.
pub fn tag(tag: &str) -> impl Fn(Input) -> LexResult + '_
where {
    move |i: Input| {
        let is_identchar = peek(identifier_character)(Input::new(tag.clone())).is_ok();
        let (i, res) = nom_tag(tag)(i)?;
        if is_identchar {
            peek(not(identifier_character))(i.clone())?;
        }
        Ok((i, res))
    }
}
