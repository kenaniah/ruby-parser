use crate::lexers::identifier::*;
use crate::lexers::numeric::{hexadecimal_digit, octal_digit};
use crate::lexers::program::*;
use crate::{
    CharResult, Input, Interpolatable, InterpolatableResult, ParseResult, Segment, SegmentResult,
    StringResult, TokenResult,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{map, not, peek, recognize};
use nom::multi::{many0, many1, many_m_n, separated_list1};
use nom::sequence::{preceded, tuple};
use std::convert::TryFrom;

type DelimitedInput<'a> = (Input<'a>, Option<char>);
type DelimitedStringResult<'a> = nom::IResult<(Input<'a>, Option<char>), String>;
type DelimitedCharResult<'a> = nom::IResult<(Input<'a>, Option<char>), char>;

/// `%q` *non_expanded_delimited_string*
pub(crate) fn quoted_non_expanded_literal_string(i: Input) -> StringResult {
    preceded(tag("%q"), non_expanded_delimited_string)(i)
}

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn non_expanded_delimited_string(i: Input) -> StringResult {
    match _non_expanded_delimited_string((i, None)) {
        Ok(((i, _), str)) => Ok((i, str)),
        Err(_) => Err(nom::Err::Error((i, crate::ErrorKind::Char)))
    }
}

fn _non_expanded_delimited_string(i: DelimitedInput) -> DelimitedStringResult {
    let (mut i, delimiter) = literal_beginning_delimiter(i)?;
    i.1 = Some(delimiter);
    let (i, contents) = non_expanded_literal_string(i)?;
    let (i, _) = literal_ending_delimiter(i)?;
    Ok((i, contents))
}

/// *non_expanded_literal_character* | *non_expanded_delimited_string*
pub(crate) fn non_expanded_literal_string(i: DelimitedInput) -> DelimitedStringResult {
    alt((
        _non_expanded_delimited_string,
        _non_expanded_delimited_string
    ))(i)
    // alt((
    //     non_expanded_literal_character,
    //     _non_expanded_delimited_string,
    // ))(i)
}

/// *non_escaped_literal_character* | *non_expanded_literal_escape_sequence*
pub(crate) fn non_expanded_literal_character(i: DelimitedInput) -> DelimitedStringResult {
    stub(i)
}

/// *source_character* **but not** *quoted_literal_escape_character*
pub(crate) fn non_escaped_literal_character(i: DelimitedInput) -> DelimitedStringResult {
    stub(i)
}

/// *non_expanded_literal_escape_character_sequence* | *non_escaped_non_expanded_literal_character_sequence*
pub(crate) fn non_expanded_literal_escape_sequence(i: DelimitedInput) -> DelimitedStringResult {
    stub(i)
}

/// `\` *non_expanded_literal_escaped_character*
pub(crate) fn non_expanded_literal_escape_character_sequence(i: DelimitedInput) -> DelimitedStringResult {
    stub(i)
}

/// *literal_beginning_delimiter* | *literal_ending_delimiter* | `\`
pub(crate) fn non_expanded_literal_escaped_character(i: DelimitedInput) -> DelimitedStringResult {
    stub(i)
}

/// *non_expanded_literal_escaped_character*
pub(crate) fn quoted_literal_escape_character(i: DelimitedInput) -> DelimitedStringResult {
    non_expanded_literal_escaped_character(i)
}

/// `\` *non_escaped_non_expanded_literal_character*
pub(crate) fn non_escaped_non_expanded_literal_character_sequence(i: DelimitedInput) -> DelimitedStringResult {
    stub(i)
}

/// *source_character* **but not** *non_expanded_literal_escaped_character*
pub(crate) fn non_escaped_non_expanded_literal_character(i: DelimitedInput) -> DelimitedStringResult {
    stub(i)
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_beginning_delimiter(i: DelimitedInput) -> DelimitedCharResult {
    stub_char(i)
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_ending_delimiter(i: DelimitedInput) -> DelimitedCharResult {
    stub_char(i)
}

fn stub(i: DelimitedInput) -> DelimitedStringResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

fn stub_char(i: DelimitedInput) -> DelimitedCharResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
