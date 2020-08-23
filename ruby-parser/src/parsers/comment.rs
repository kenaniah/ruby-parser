//! Provides parsers for comments

use crate::parsers::program::*;
use crate::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{not, opt, peek, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, tuple};

/// *single_line_comment* | *multi_line_comment*
pub(crate) fn comment(i: Input) -> TokenResult {
    let (i, content) = alt((single_line_comment, multi_line_comment))(i)?;
    Ok((i, Token::Comment((*content).to_owned())))
}

/// `#` *comment_content*?
pub(crate) fn single_line_comment(i: Input) -> ParseResult {
    recognize(tuple((char('#'), opt(comment_content))))(i)
}

/// *line_content*
pub(crate) fn comment_content(i: Input) -> ParseResult {
    line_content(i)
}

/// ( *source_character*+ ) **but not** ( *source_character** *line_terminator* *source_character** )
pub(crate) fn line_content(i: Input) -> ParseResult {
    recognize(many1(_line_content))(i)
}

fn _line_content(i: Input) -> CharResult {
    preceded(peek(not(line_terminator)), source_character)(i)
}

/// *multi_line_comment_begin_line* *multi_line_comment_line** *multi_line_comment_end_line*
pub(crate) fn multi_line_comment(i: Input) -> ParseResult {
    recognize(tuple((
        multi_line_comment_begin_line,
        many0(multi_line_comment_line),
        multi_line_comment_end_line,
    )))(i)
}

/// [ beginning of a line ] `=begin` *rest_of_begin_end_line*? *line_terminator*
pub(crate) fn multi_line_comment_begin_line(i: Input) -> ParseResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, crate::ErrorKind::Space)));
    }
    let (i, _) = tag("=begin")(i)?;
    recognize(tuple((opt(rest_of_begin_end_line), line_terminator)))(i)
}

/// [ beginning of a line ] `=end` *rest_of_begin_end_line*? ( *line_terminator* | [ end of a program ] )
pub(crate) fn multi_line_comment_end_line(i: Input) -> ParseResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, crate::ErrorKind::Space)));
    }
    let (i, _) = tag("=end")(i)?;
    recognize(tuple((opt(rest_of_begin_end_line), opt(line_terminator))))(i)
}

/// *whitespace*+ *comment_content*
pub(crate) fn rest_of_begin_end_line(i: Input) -> ParseResult {
    recognize(tuple((many1(whitespace), comment_content)))(i)
}

/// *comment_line* **but not** *multi_line_comment_end_line*
pub(crate) fn multi_line_comment_line(i: Input) -> ParseResult {
    preceded(peek(not(multi_line_comment_end_line)), comment_line)(i)
}

/// *comment_content*? *line_terminator*
pub(crate) fn comment_line(i: Input) -> ParseResult {
    recognize(tuple((opt(comment_content), line_terminator)))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_comment() {
        fn output(x: Input) -> &str {
            *x
        }
        use_parser!(single_line_comment, output);
        // Parse errors
        assert_err!("");
        assert_err!("foobar");
        assert_err!("  # meh");
        assert_err!("#\n");
        assert_err!("# Newline should not be consumed\n");
        // Success cases
        assert_ok!("#", "#");
        assert_ok!("# No newline", "# No newline");
        assert_ok!("#This is a comment", "#This is a comment");
        assert_partial!("# With newline\nfoobar\n", "# With newline");
    }

    #[test]
    fn test_multi_line_comment() {
        use_parser!(multi_line_comment);
        // Parse errors
        assert_err!("  =begin\n=end");
        assert_err!("=begin\n  =end");
        assert_err!("=begin\n");
        assert_err!("=begins\n=end");
        assert_err!("=begin\nanother line\n  =end");
        // Success cases
        assert_ok!("=begin\n=end");
        assert_ok!("=begin\n=end\n");
        assert_ok!("=begin\r\n=end extra");
        assert_ok!("=begin\n=end extra\r\n");
        assert_ok!("=begin extra\n=end extra\n");
        assert_ok!("=begin extra\n\tcontent line\n=end extra\n");
        assert_ok!("=begin extra\n\tcontent line\n\n\nanother_line\n\n=end extra\n");
    }
}
