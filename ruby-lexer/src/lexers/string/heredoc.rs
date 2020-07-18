use crate::lexers::identifier::identifier_character;
use crate::lexers::program::{line_terminator, source_character};
use crate::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, not, opt, peek, recognize};
use nom::multi::many1;
use nom::sequence::{delimited, preceded};

/// *heredoc_start_line* *heredoc_body* *heredoc_end_line*
pub(crate) fn here_document(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *heredoc_signifier* *rest_of_line*
pub(crate) fn heredoc_start_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// `<<` *heredoc_quote_type_specifier*
pub(crate) fn heredoc_signifier(i: Input) -> ParseResult {
    preceded(tag("<<"), heredoc_quote_type_specifier)(i)
}

/// *line_content*? *line_terminator*
pub(crate) fn rest_of_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *heredoc_body_line**
pub(crate) fn heredoc_body(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *comment_line* **but not** *heredoc_end_line*
pub(crate) fn heredoc_body_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// ( `-` | `~` )? *heredoc_quote_type*
pub(crate) fn heredoc_quote_type_specifier(i: Input) -> ParseResult {
    preceded(
        set_indentiation(opt(alt((char('-'), char('~'))))),
        heredoc_quote_type,
    )(i)
}

/// *non_quoted_delimiter* | *single_quoted_delimiter* | *double_quoted_delimiter* | *command_quoted_delimiter*
pub(crate) fn heredoc_quote_type(i: Input) -> ParseResult {
    let (mut i, res) = alt((
        set_quote_type(non_quoted_delimiter, HeredocQuoteType::Unquoted),
        set_quote_type(single_quoted_delimiter, HeredocQuoteType::SingleQuoted),
        set_quote_type(double_quoted_delimiter, HeredocQuoteType::DoubleQuoted),
        set_quote_type(command_quoted_delimiter, HeredocQuoteType::CommandQuoted),
    ))(i)?;
    i.metadata.heredoc_identifier = Some(*res);
    Ok((i, res))
}

/// *non_quoted_delimiter_identifier*
pub(crate) fn non_quoted_delimiter(i: Input) -> ParseResult {
    non_quoted_delimiter_identifier(i)
}

/// *identifier_character*+
pub(crate) fn non_quoted_delimiter_identifier(i: Input) -> ParseResult {
    recognize(many1(identifier_character))(i)
}

/// `'` *single_quoted_delimiter_identifier* `'`
pub(crate) fn single_quoted_delimiter(i: Input) -> ParseResult {
    delimited(char('\''), single_quoted_delimiter_identifier, char('\''))(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( `'` | *line_terminator* ) )*
pub(crate) fn single_quoted_delimiter_identifier(i: Input) -> ParseResult {
    recognize(many1(preceded(
        peek(not(alt((tag("'"), line_terminator)))),
        source_character,
    )))(i)
}

/// `"` *double_quoted_delimiter_identifier* `"`
pub(crate) fn double_quoted_delimiter(i: Input) -> ParseResult {
    delimited(char('"'), double_quoted_delimiter_identifier, char('"'))(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( `"` | *line_terminator* ) )*
pub(crate) fn double_quoted_delimiter_identifier(i: Input) -> ParseResult {
    recognize(many1(preceded(
        peek(not(alt((tag("\""), line_terminator)))),
        source_character,
    )))(i)
}

/// ``` *command_quoted_delimiter_identifier* ```
pub(crate) fn command_quoted_delimiter(i: Input) -> ParseResult {
    delimited(char('`'), command_quoted_delimiter_identifier, char('`'))(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( ``` | *line_terminator* ) )*
pub(crate) fn command_quoted_delimiter_identifier(i: Input) -> ParseResult {
    recognize(many1(preceded(
        peek(not(alt((tag("`"), line_terminator)))),
        source_character,
    )))(i)
}

/// *indented_heredoc_end_line* | *non_indented_heredoc_end_line*
pub(crate) fn heredoc_end_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// [ beginning of a line ] *whitespace** *heredoc_quote_type_identifier* *line_terminator*
pub(crate) fn indented_heredoc_end_line(i: Input) -> InterpolatableResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, crate::ErrorKind::Space)));
    }
    stub(i)
}

/// [ beginning of a line ] *heredoc_quote_type_identifier* *line_terminator*
pub(crate) fn non_indented_heredoc_end_line(i: Input) -> InterpolatableResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, crate::ErrorKind::Space)));
    }
    stub(i)
}

/// *non_quoted_delimiter_identifier* | *single_quoted_delimiter_identifier* | *double_quoted_delimiter_identifier* | *command_quoted_delimiter_identifier*
pub(crate) fn heredoc_quote_type_identifier(i: Input) -> ParseResult {
    stub_p(i)
}

/// Sets the type of heredoc indentation used
fn set_indentiation<'a, E, F>(
    mut func: F,
) -> impl FnMut(Input<'a>) -> nom::IResult<Input<'a>, Option<char>, E>
where
    F: nom::Parser<Input<'a>, Option<char>, E>,
{
    move |mut i: Input<'a>| {
        let res = func.parse(i);
        match res {
            Ok((mut i, char)) => {
                i.metadata.heredoc_indentation = match char {
                    Some('-') => Some(HeredocIndentation::Indented),
                    Some('~') => Some(HeredocIndentation::FullyIntented),
                    _ => Some(HeredocIndentation::Unindented),
                };
                Ok((i, char))
            }
            error @ _ => error,
        }
    }
}

/// Sets the type of heredoc quoting used
fn set_quote_type<'a, O1, E, F>(
    mut func: F,
    quote_type: HeredocQuoteType,
) -> impl FnMut(Input<'a>) -> nom::IResult<Input<'a>, O1, E>
where
    F: nom::Parser<Input<'a>, O1, E>,
{
    move |mut i: Input<'a>| {
        let res = func.parse(i);
        match res {
            Ok((mut i, o1)) => {
                i.metadata.heredoc_quote_type = Some(quote_type);
                Ok((i, o1))
            }
            error @ _ => error,
        }
    }
}

/// Manages the state of the input's heredoc parsing
fn wrap_heredoc<'a, O1, E, F>(
    mut func: F,
) -> impl FnMut(Input<'a>) -> nom::IResult<Input<'a>, O1, E>
where
    F: nom::Parser<Input<'a>, O1, E>,
{
    move |mut i: Input<'a>| {
        let quote = i.metadata.heredoc_quote_type;
        let delim = i.metadata.heredoc_identifier;
        let indent = i.metadata.heredoc_indentation;
        i.metadata.heredoc_quote_type = None;
        i.metadata.heredoc_identifier = None;
        i.metadata.heredoc_indentation = None;
        let res = func.parse(i);
        match res {
            Ok((mut i, o1)) => {
                i.metadata.heredoc_quote_type = quote;
                i.metadata.heredoc_identifier = delim;
                i.metadata.heredoc_indentation = indent;
                Ok((i, o1))
            }
            error @ _ => error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // macro_rules! assert_string {
    //     ($a:expr, $b:expr) => {
    //         parser($input.into())
    //         assert_ok!($a, Interpolatable::String($b.to_owned()))
    //     };
    // }
    // macro_rules! assert_interpolated {
    //     ($a:expr, $b:expr) => {
    //         assert_ok!($a, Interpolatable::Interpolated($b))
    //     };
    // }

    #[test]
    fn test_heredoc_quote_type() {
        use_parser!(heredoc_quote_type);
    }
}

fn stub_p(i: Input) -> ParseResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

fn stub_s(i: Input) -> StringResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

fn stub(i: Input) -> InterpolatableResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
