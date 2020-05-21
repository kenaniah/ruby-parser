//! Provides parsers for comments

use crate::{CharResult, Input, StringResult};
use nom::branch::alt;
use nom::character::complete::{anychar, char, line_ending, one_of};
use nom::combinator::map;

/// *single_line_comment* | *multi_line_comment*
pub fn comment(i: Input) -> StringResult {
    alt((single_line_comment, multi_line_comment))(i)
}

/// `#` *comment_content*?
pub fn single_line_comment(i: Input) -> StringResult {
    stub_string(i)
}

/// *line_content*
pub fn comment_content(i: Input) -> StringResult {
    line_content(i)
}

/// ( *source_character*+ ) **but not** ( *source_character** *line_terminator* *source_character** )
pub fn line_content(i: Input) -> StringResult {
    stub_string(i)
}

/// *multi_line_comment_begin_line* *multi_line_comment_line*? *multi_line_comment_end_line*
pub fn multi_line_comment(i: Input) -> StringResult {
    stub_string(i)
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
