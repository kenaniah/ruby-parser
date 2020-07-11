use crate::{
    Input, Interpolatable, InterpolatableResult, ParseResult, Segment, SegmentResult, StringResult,
    TrackedLocation,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char};
use nom::combinator::verify;
use nom::combinator::{map, not, peek};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};

type DelimitedInput<'a> = TrackedLocation<&'a str, Option<char>>;
type DelimitedStringResult<'a> = nom::IResult<DelimitedInput<'a>, String>;
type DelimitedCharResult<'a> = nom::IResult<DelimitedInput<'a>, char>;

impl DelimitedInput<'_> {
    fn start_delimiter(&self) -> Option<char> {
        self.metadata
    }
    fn end_delimiter(&self) -> Option<char> {
        match self.metadata {
            Some('{') => Some('}'),
            Some('(') => Some(')'),
            Some('[') => Some(']'),
            Some('<') => Some('>'),
            _ => self.metadata,
        }
    }
}

/// `%q` *non_expanded_delimited_string*
pub(crate) fn quoted_non_expanded_literal_string(i: Input) -> StringResult {
    preceded(tag("%q"), non_expanded_delimited_string)(i)
}

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn non_expanded_delimited_string(i: Input) -> StringResult {
    let meta = i.metadata;
    let di: DelimitedInput = DelimitedInput::new_with_pos(*i, i.offset(), i.line(), i.char());
    match _non_expanded_delimited_string(di) {
        Ok((di, str)) => Ok((
            Input::new_with_pos_and_meta(*di, di.offset(), di.line(), di.char(), meta),
            str,
        )),
        Err(_) => Err(nom::Err::Error((
            Input::new_with_pos_and_meta(*di, di.offset(), di.line(), di.char(), meta),
            crate::ErrorKind::Char,
        ))),
    }
}

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
fn _non_expanded_delimited_string(i: DelimitedInput) -> DelimitedStringResult {
    map(
        delimited(
            literal_beginning_delimiter,
            many0(non_expanded_literal_string),
            literal_ending_delimiter,
        ),
        |vec| {
            let mut s = String::new();
            for str in vec {
                s.push_str(&str);
            }
            s
        },
    )(i)
}

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
fn _non_expanded_delimited_string2(i: DelimitedInput) -> DelimitedStringResult {
    map(
        tuple((
            literal_beginning_delimiter,
            many0(non_expanded_literal_string),
            literal_ending_delimiter,
        )),
        |t| {
            let mut s = String::new();
            s.push(t.0);
            for str in t.1 {
                s.push_str(&str);
            }
            s.push(t.2);
            s
        },
    )(i)
}

/// *non_expanded_literal_character* | *non_expanded_delimited_string*
pub(crate) fn non_expanded_literal_string(i: DelimitedInput) -> DelimitedStringResult {
    alt((
        non_expanded_literal_character,
        _non_expanded_delimited_string2,
    ))(i)
}

/// *non_escaped_literal_character* | *non_expanded_literal_escape_sequence*
pub(crate) fn non_expanded_literal_character(i: DelimitedInput) -> DelimitedStringResult {
    alt((
        non_escaped_literal_character,
        non_expanded_literal_escape_sequence,
    ))(i)
}

/// *source_character* **but not** *quoted_literal_escape_character*
pub(crate) fn non_escaped_literal_character(i: DelimitedInput) -> DelimitedStringResult {
    peek(not(quoted_literal_escape_character))(i)?;
    map(anychar, |c| c.to_string())(i)
}

/// *non_expanded_literal_escape_character_sequence* | *non_escaped_non_expanded_literal_character_sequence*
pub(crate) fn non_expanded_literal_escape_sequence(i: DelimitedInput) -> DelimitedStringResult {
    alt((
        non_expanded_literal_escape_character_sequence,
        non_escaped_non_expanded_literal_character_sequence,
    ))(i)
}

/// `\` *non_expanded_literal_escaped_character*
pub(crate) fn non_expanded_literal_escape_character_sequence(
    i: DelimitedInput,
) -> DelimitedStringResult {
    preceded(char('\\'), non_expanded_literal_escaped_character)(i)
}

/// *literal_beginning_delimiter* | *literal_ending_delimiter* | `\`
pub(crate) fn non_expanded_literal_escaped_character(i: DelimitedInput) -> DelimitedStringResult {
    map(
        alt((
            literal_beginning_delimiter,
            literal_ending_delimiter,
            char('\\'),
        )),
        |c| c.to_string(),
    )(i)
}

/// *non_expanded_literal_escaped_character*
pub(crate) fn quoted_literal_escape_character(i: DelimitedInput) -> DelimitedStringResult {
    non_expanded_literal_escaped_character(i)
}

/// `\` *non_escaped_non_expanded_literal_character*
pub(crate) fn non_escaped_non_expanded_literal_character_sequence(
    i: DelimitedInput,
) -> DelimitedStringResult {
    map(
        tuple((char('\\'), non_escaped_non_expanded_literal_character)),
        |t| {
            let mut s = String::new();
            s.push(t.0);
            s.push_str(&t.1);
            s
        }
    )(i)
}

/// *source_character* **but not** *non_expanded_literal_escaped_character*
pub(crate) fn non_escaped_non_expanded_literal_character(
    i: DelimitedInput,
) -> DelimitedStringResult {
    peek(not(non_expanded_literal_escaped_character))(i)?;
    map(anychar, |c| c.to_string())(i)
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_beginning_delimiter(i: DelimitedInput) -> DelimitedCharResult {
    let (mut i, c) = match i.start_delimiter() {
        Some(c) => char(c)(i)?,
        None => verify(anychar, |c: &char| c.is_ascii_punctuation())(i)?,
    };
    i.metadata = Some(c);
    Ok((i, c))
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_ending_delimiter(i: DelimitedInput) -> DelimitedCharResult {
    match i.end_delimiter() {
        Some(c) => char(c)(i),
        None => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quoted_non_expanded_literal_string() {
        use_parser!(quoted_non_expanded_literal_string);
        // Parse errors
        assert_err!("%q(");
        assert_err!("%q((");
        assert_err!("%q(");
        assert_err!("%q:");
        assert_err!("%q{foo");
        assert_err!("%q(foo)bar");
        assert_err!("%q[[abc] [def]");
        // Success cases
        assert_ok!("%q()", "");
        assert_ok!("%q(foobar)", "foobar");
        assert_ok!("%q<\0>", "\0");
        assert_ok!("%q:foo\nbar:", "foo\nbar");
        assert_ok!("%q:foo\\n\\:bar\\\\:", "foo\\n:bar\\");
        assert_ok!("%q%Smiley 😂 here!%", "Smiley 😂 here!");
        assert_ok!("%q[[abc] [def]]", "[abc] [def]");
        assert_ok!("%q[\\[abc\\)def(]", "[abc\\)def(");
        assert_ok!("%q{{{\\{}}}", "{{{}}");
    }

    #[test]
    fn test_literal_beginning_delimiter() {
        use_parser!(literal_beginning_delimiter);
        // Parse errors
        assert_err!("");
        assert_err!("a");
        assert_err!(" ");
        assert_err!("5");
        assert_err!("東"); // U+6771: 'CJK Unified Ideograph-6771' "East"
        assert_err!("\n");
        // Success cases
        assert_ok!("<", '<');
        assert_ok!("(", '(');
        assert_ok!("}", '}');
        assert_ok!(":", ':');
        assert_ok!("_", '_');
        assert_ok!("\\", '\\');
    }
}
