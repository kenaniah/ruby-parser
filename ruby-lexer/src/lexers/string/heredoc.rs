use crate::lexers::comment::line_content;
use crate::lexers::identifier::identifier_character;
use crate::lexers::program::{line_terminator, source_character, whitespace};
use crate::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, not, opt, peek, recognize};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, terminated};

/// *heredoc_start_line* *heredoc_body* *heredoc_end_line*
pub(crate) fn here_document(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *heredoc_signifier* *rest_of_line*
fn heredoc_start_line(i: Input) -> ParseResult {
    preceded(heredoc_signifier, rest_of_line)(i)
}

/// `<<` *heredoc_quote_type_specifier*
fn heredoc_signifier(i: Input) -> ParseResult {
    preceded(tag("<<"), heredoc_quote_type_specifier)(i)
}

/// *line_content*? *line_terminator*
fn rest_of_line(i: Input) -> ParseResult {
    terminated(recognize(opt(line_content)), line_terminator)(i)
}

/// *heredoc_body_line**
fn heredoc_body(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *comment_line* **but not** *heredoc_end_line*
fn heredoc_body_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// ( `-` | `~` )? *heredoc_quote_type*
fn heredoc_quote_type_specifier(i: Input) -> ParseResult {
    preceded(
        set_indentiation(opt(alt((char('-'), char('~'))))),
        heredoc_quote_type,
    )(i)
}

/// *non_quoted_delimiter* | *single_quoted_delimiter* | *double_quoted_delimiter* | *command_quoted_delimiter*
fn heredoc_quote_type(i: Input) -> ParseResult {
    let (mut i, res) = alt((
        set_quote_type(non_quoted_delimiter, HeredocQuoteType::Unquoted),
        set_quote_type(single_quoted_delimiter, HeredocQuoteType::SingleQuoted),
        set_quote_type(double_quoted_delimiter, HeredocQuoteType::DoubleQuoted),
        set_quote_type(command_quoted_delimiter, HeredocQuoteType::CommandQuoted),
    ))(i)?;
    i.metadata.heredoc.as_deref_mut().unwrap().identifier = Some(*res);
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
    preceded(
        peek(not(whitespace)),
        recognize(many1(preceded(
            peek(not(alt((tag("'"), line_terminator)))),
            source_character,
        ))),
    )(i)
}

/// `"` *double_quoted_delimiter_identifier* `"`
pub(crate) fn double_quoted_delimiter(i: Input) -> ParseResult {
    delimited(char('"'), double_quoted_delimiter_identifier, char('"'))(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( `"` | *line_terminator* ) )*
pub(crate) fn double_quoted_delimiter_identifier(i: Input) -> ParseResult {
    preceded(
        peek(not(whitespace)),
        recognize(many1(preceded(
            peek(not(alt((tag("\""), line_terminator)))),
            source_character,
        ))),
    )(i)
}

/// ``` *command_quoted_delimiter_identifier* ```
pub(crate) fn command_quoted_delimiter(i: Input) -> ParseResult {
    delimited(char('`'), command_quoted_delimiter_identifier, char('`'))(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( ``` | *line_terminator* ) )*
pub(crate) fn command_quoted_delimiter_identifier(i: Input) -> ParseResult {
    preceded(
        peek(not(whitespace)),
        recognize(many1(preceded(
            peek(not(alt((tag("`"), line_terminator)))),
            source_character,
        ))),
    )(i)
}

/// *indented_heredoc_end_line* | *non_indented_heredoc_end_line*
fn heredoc_end_line(i: Input) -> ParseResult {
    match i.metadata.heredoc.as_ref().unwrap().indentation {
        Some(HeredocIndentation::Unindented) => non_indented_heredoc_end_line(i.clone()),
        _ => indented_heredoc_end_line(i.clone()),
    }
}

/// [ beginning of a line ] *whitespace** *heredoc_quote_type_identifier* *line_terminator*
fn indented_heredoc_end_line(i: Input) -> ParseResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, crate::ErrorKind::Space)));
    }
    delimited(
        many0(whitespace),
        heredoc_quote_type_identifier,
        opt(line_terminator),
    )(i)
}

/// [ beginning of a line ] *heredoc_quote_type_identifier* *line_terminator*
fn non_indented_heredoc_end_line(i: Input) -> ParseResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, crate::ErrorKind::Space)));
    }
    terminated(heredoc_quote_type_identifier, opt(line_terminator))(i)
}

/// *non_quoted_delimiter_identifier* | *single_quoted_delimiter_identifier* | *double_quoted_delimiter_identifier* | *command_quoted_delimiter_identifier*
fn heredoc_quote_type_identifier(i: Input) -> ParseResult {
    if let Some(identifier) = i.metadata.heredoc.as_ref().unwrap().identifier {
        tag(identifier)(i.clone())
    } else {
        Err(nom::Err::Error((i.clone(), crate::ErrorKind::Char)))
    }
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
                i.metadata.heredoc.as_deref_mut().unwrap().indentation = match char {
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
                i.metadata.heredoc.as_deref_mut().unwrap().quote_type = Some(quote_type);
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
        let original = i.metadata.heredoc;
        i.metadata.heredoc = Some(Box::new(HeredocMetadata::default()));
        let res = func.parse(i);
        match res {
            Ok((mut i, o1)) => {
                // The following is a hack to intentionally expose the heredoc parser's state
                // for verification purposes within this module's unit tests
                #[cfg(test)]
                {
                    if original.is_none() || !original.as_deref().unwrap().should_leak {
                        i.metadata.heredoc = original;
                    }
                }
                #[cfg(not(test))]
                {
                    i.metadata.heredoc = original;
                }
                Ok((i, o1))
            }
            error @ _ => error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_signifier {
        ($input:expr, $ident:expr, $indent:expr, $quote:expr) => {
            // Bootstrap the input with a flag that will cause the heredoc parser's state to leak
            let mut i: Input = $input.into();
            i.metadata.heredoc = Some(Box::new(HeredocMetadata::default()));
            i.metadata.heredoc.as_deref_mut().unwrap().should_leak = true;
            let (i, result) = parser!(i).unwrap();
            // Verifies the state of the heredoc parser
            let heredoc = i.metadata.heredoc.as_ref().unwrap();
            assert_eq!(heredoc.identifier, Some($ident));
            assert_eq!(heredoc.indentation, Some($indent));
            assert_eq!(heredoc.quote_type, Some($quote));
        };
    }

    #[test]
    fn test_heredoc_signifier() {
        // This unit test uses a wrapped testing harness that intentionally leaks the
        // heredoc parser's top-level state
        fn wrapped_heredoc_signifier(i: Input) -> ParseResult {
            wrap_heredoc(heredoc_signifier)(i)
        }
        use_parser!(wrapped_heredoc_signifier);
        assert_err!("<<");
        assert_err!("<<FOO,");
        assert_err!("<<FOO ");
        assert_err!("<<'FOO");
        assert_err!("<<'FOO\nBAR'");
        assert_err!("<<\"bar\"\"");
        assert_err!("<<`baz");
        assert_err!("<<''");
        assert_err!("<<-' foo'");
        assert_err!("<<`baz");
        assert_signifier!(
            "<<foo",
            "foo",
            HeredocIndentation::Unindented,
            HeredocQuoteType::Unquoted
        );
        assert_signifier!(
            "<<-BAR",
            "BAR",
            HeredocIndentation::Indented,
            HeredocQuoteType::Unquoted
        );
        assert_signifier!(
            "<<~'BA Z'",
            "BA Z",
            HeredocIndentation::FullyIntented,
            HeredocQuoteType::SingleQuoted
        );
        assert_signifier!(
            "<<\"bar\"",
            "bar",
            HeredocIndentation::Unindented,
            HeredocQuoteType::DoubleQuoted
        );
        assert_signifier!(
            "<<-`FOO,  `",
            "FOO,  ",
            HeredocIndentation::Indented,
            HeredocQuoteType::CommandQuoted
        );
        assert_signifier!(
            "<<'foo :bar'",
            "foo :bar",
            HeredocIndentation::Unindented,
            HeredocQuoteType::SingleQuoted
        );
    }

    #[test]
    fn test_heredoc_start_line() {
        fn wrapped_heredoc_start_line(i: Input) -> StringResult {
            map(wrap_heredoc(heredoc_start_line), |s| (*s).to_owned())(i)
        }
        use_parser!(wrapped_heredoc_start_line);
        assert_err!("<<-Foo");
        assert_err!("<<-Foo\nbar\n");
        assert_ok!("<<-Foo\n", "");
        assert_ok!("<<-FOO BAR\n", " BAR");
        assert_ok!("<<foo, 2; 3 * blah\n", ", 2; 3 * blah");
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
