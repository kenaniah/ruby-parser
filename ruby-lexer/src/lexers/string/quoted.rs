use crate::lexers::string::double::double_escape_sequence;
use crate::lexers::string::double::interpolated_character_sequence;
use crate::types::CharResult;
use crate::{
    Input, Interpolatable, InterpolatableResult, Metadata, ParseResult, Segment, SegmentResult,
    StringResult,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char};
use nom::combinator::verify;
use nom::combinator::{map, not, peek};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};

/// `%q` *non_expanded_delimited_string*
pub(crate) fn quoted_non_expanded_literal_string(i: Input) -> StringResult {
    preceded(tag("%q"), non_expanded_delimited_string)(i)
}

/// `%` `Q`? *expanded_delimited_string*
// pub(crate) fn quoted_expanded_literal_string(i: Input) -> StringResult {
//     preceded(alt((tag("%Q"), tag("%"))), expanded_delimited_string)(i)
// }

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn non_expanded_delimited_string(i: Input) -> StringResult {
    let delim = i.metadata.quote_delimiter;
    let (mut i, str) = map(
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
    )(i)?;
    i.metadata.quote_delimiter = delim;
    Ok((i, str))
}

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
fn _non_expanded_delimited_string(i: Input) -> StringResult {
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

/// *literal_beginning_delimiter* *expanded_literal_string** *literal_ending_delimiter*
// pub(crate) fn expanded_delimited_string(i: Input) -> StringResult {
//     let (mut i, str) = map(
//         delimited(
//             literal_beginning_delimiter,
//             many0(expanded_literal_string),
//             literal_ending_delimiter,
//         ),
//         |vec| {
//             let mut s = String::new();
//             for str in vec {
//                 s.push_str(&str);
//             }
//             s
//         },
//     )(i)?;
//     i.metadata.quote_delimiter = None;
//     Ok((i, str))
// }

/// *literal_beginning_delimiter* *expanded_literal_string** *literal_ending_delimiter*
// fn _expanded_delimited_string(i: Input) -> InterpolatableResult {
//     map(
//         tuple((
//             literal_beginning_delimiter,
//             many0(expanded_literal_string),
//             literal_ending_delimiter,
//         )),
//         |t| {
//             Interpolatable::from(t.1)
//         },
//     )(i)
// }

/// *non_expanded_literal_character* | *non_expanded_delimited_string*
pub(crate) fn non_expanded_literal_string(i: Input) -> StringResult {
    alt((
        non_expanded_literal_character,
        _non_expanded_delimited_string,
    ))(i)
}

/// *expanded_literal_character* | *expanded_delimited_string*
// pub(crate) fn expanded_literal_string(i: Input) -> SegmentResult {
//     alt((expanded_literal_character, _expanded_delimited_string))(i)
// }

/// *non_escaped_literal_character* | *non_expanded_literal_escape_sequence*
pub(crate) fn non_expanded_literal_character(i: Input) -> StringResult {
    alt((
        non_escaped_literal_character,
        non_expanded_literal_escape_sequence,
    ))(i)
}

/// *non_escaped_literal_character* **but not** `#` | `#` **not** ( `$` | `@` | `{` ) | *double_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn expanded_literal_character(i: Input) -> SegmentResult {
    alt((
        map(
            preceded(peek(not(char('#'))), non_escaped_literal_character),
            |s| Segment::String(s),
        ),
        map(double_escape_sequence, |s| Segment::String(s)),
        map(interpolated_character_sequence, |e| Segment::Expr(e)),
        map(char('#'), |c| Segment::Char(c)),
    ))(i)
}

/// *source_character* **but not** *quoted_literal_escape_character*
pub(crate) fn non_escaped_literal_character(i: Input) -> StringResult {
    peek(not(quoted_literal_escape_character))(i)?;
    map(anychar, |c| c.to_string())(i)
}

/// *non_expanded_literal_escape_character_sequence* | *non_escaped_non_expanded_literal_character_sequence*
pub(crate) fn non_expanded_literal_escape_sequence(i: Input) -> StringResult {
    alt((
        non_expanded_literal_escape_character_sequence,
        non_escaped_non_expanded_literal_character_sequence,
    ))(i)
}

/// `\` *non_expanded_literal_escaped_character*
pub(crate) fn non_expanded_literal_escape_character_sequence(i: Input) -> StringResult {
    preceded(char('\\'), non_expanded_literal_escaped_character)(i)
}

/// *literal_beginning_delimiter* | *literal_ending_delimiter* | `\`
pub(crate) fn non_expanded_literal_escaped_character(i: Input) -> StringResult {
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
pub(crate) fn quoted_literal_escape_character(i: Input) -> StringResult {
    non_expanded_literal_escaped_character(i)
}

/// `\` *non_escaped_non_expanded_literal_character*
pub(crate) fn non_escaped_non_expanded_literal_character_sequence(i: Input) -> StringResult {
    map(
        tuple((char('\\'), non_escaped_non_expanded_literal_character)),
        |t| {
            let mut s = String::new();
            s.push(t.0);
            s.push_str(&t.1);
            s
        },
    )(i)
}

/// *source_character* **but not** *non_expanded_literal_escaped_character*
pub(crate) fn non_escaped_non_expanded_literal_character(i: Input) -> StringResult {
    peek(not(non_expanded_literal_escaped_character))(i)?;
    map(anychar, |c| c.to_string())(i)
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_beginning_delimiter(i: Input) -> CharResult {
    let (mut i, c) = match start_delimiter(&i.metadata) {
        Some(c) => char(c)(i)?,
        None => verify(anychar, |c: &char| c.is_ascii_punctuation())(i)?,
    };
    i.metadata.quote_delimiter = Some(c);
    Ok((i, c))
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_ending_delimiter(i: Input) -> CharResult {
    match end_delimiter(&i.metadata) {
        Some(c) => char(c)(i),
        None => unimplemented!(),
    }
}

fn start_delimiter(meta: &Metadata) -> Option<char> {
    meta.quote_delimiter
}

fn end_delimiter(meta: &Metadata) -> Option<char> {
    match meta.quote_delimiter {
        Some('{') => Some('}'),
        Some('(') => Some(')'),
        Some('[') => Some(']'),
        Some('<') => Some('>'),
        _ => meta.quote_delimiter,
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
        assert_err!("%q[[abc] [def])");
        assert_err!("%q((abc\\))");
        // Success cases
        assert_ok!("%q()", "");
        assert_ok!("%q((abc))", "(abc)");
        assert_ok!("%q((abc\\)))", "(abc))");
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
