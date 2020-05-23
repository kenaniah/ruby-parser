//! Provides parsers for comments

use crate::lexers::program::line_terminator;
use crate::lexers::program::source_character;
use crate::{Input, StringResult};
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::combinator::recognize;
use nom::multi::many_till;
use nom::sequence::tuple;

/// *single_line_comment* | *multi_line_comment*
pub fn comment(i: Input) -> StringResult {
    alt((single_line_comment, multi_line_comment))(i)
}

/// `#` *comment_content*?
pub fn single_line_comment(i: Input) -> StringResult {
    map(recognize(tuple((char('#'), opt(comment_content)))), |s| {
        s.to_owned()
    })(i)
}

/// *line_content*
pub fn comment_content(i: Input) -> StringResult {
    line_content(i)
}

/// ( *source_character*+ ) **but not** ( *source_character** *line_terminator* *source_character** )
pub fn line_content(i: Input) -> StringResult {
    map(
        many_till(source_character, peek(line_terminator)),
        |chars| chars.0.into_iter().collect::<String>(),
    )(i)
}

/// *multi_line_comment_begin_line* *multi_line_comment_line*? *multi_line_comment_end_line*
pub fn multi_line_comment(i: Input) -> StringResult {
    map(
        recognize(tuple((
            multi_line_comment_begin_line,
            opt(multi_line_comment_line),
            multi_line_comment_end_line,
        ))),
        |s| s.to_owned(),
    )(i)
}

/// [ beginning of a line ] `=begin` *rest_of_begin_line*? *line_terminator*
pub fn multi_line_comment_begin_line(i: Input) -> StringResult {
    stub_string(i)
}

/// [ beginning of a line ] `=end` *rest_of_begin_end_line*? ( *line_terminator* | [ end of a program ] )
pub fn multi_line_comment_end_line(i: Input) -> StringResult {
    stub_string(i)
}

/// *whitespace*+ *comemnt_content*
pub fn rest_of_begin_end_line(i: Input) -> StringResult {
    stub_string(i)
}

/// *comment_line* **but not** *multi_line_comment_end_line*
pub fn multi_line_comment_line(i: Input) -> StringResult {
    stub_string(i)
}

/// *comment_content* *line_terminator*
pub fn comment_line(i: Input) -> StringResult {
    stub_string(i)
}

pub fn stub_string(i: Input) -> StringResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Complete)))
}
