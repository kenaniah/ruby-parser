use crate::lexers::numeric::{hexadecimal_digit, octal_digit};
use crate::lexers::program::{line_terminator, line_terminator_escape_sequence};
use crate::{CharResult, Input, StringResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{map, not, opt, peek, recognize, verify};
use nom::multi::many0;
use nom::sequence::tuple;

/// `"` *double_quoted_string_character** `"`
pub(crate) fn double_quoted_string(i: Input) -> StringResult {
    let (i, _) = char('"')(i)?;
    let (i, contents) = many0(double_quoted_string_character)(i)?;
    let (i, _) = char('"')(i)?;
    let mut string = String::new();
    for s in contents {
        string.push_str(&s);
    }
    Ok((i, string))
}

/// *source_character* **but not** ( `"` | `#` | `\` ) | `#` **not** ( `$` | `@` | `{` ) | *double_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn double_quoted_string_character(i: Input) -> StringResult {
    alt((
        map(none_of("\"#\\"), |char| char.to_string()),
        map(
            recognize(tuple((char('#'), none_of("$@{}")))),
            |s: Input| (*s).to_owned(),
        ),
        double_escape_sequence,
        interpolated_character_sequence,
    ))(i)
}

/// *simple_escape_sequence* | *non_escaped_sequence* | *line_terminator_escape_sequence* | *octal_escape_sequence* | *hexadecimal_escape_sequence* | *control_escape_sequence*
pub(crate) fn double_escape_sequence(i: Input) -> StringResult {
    alt((
        simple_escape_sequence,
        non_escaped_sequence,
        line_terminator_escape_sequence,
        octal_escape_sequence,
        hexadecimal_escape_sequence,
        control_escape_sequence,
    ))(i)
}

/// `\` *double_escaped_character*
pub(crate) fn simple_escape_sequence(i: Input) -> StringResult {
    map(
        recognize(tuple((char('\\'), double_escaped_character))),
        |s| (*s).to_owned(),
    )(i)
}

/// `\` | `n` | `t` | `r` | `f` | `v` | `a` | `e` | `b` | `s`
pub(crate) fn double_escaped_character(i: Input) -> CharResult {
    one_of("\\ntrfvaebs")(i)
}

/// `\` *non_escaped_double_quoted_string_char*
pub(crate) fn non_escaped_sequence(i: Input) -> StringResult {
    map(
        recognize(tuple((char('\\'), non_escaped_double_quoted_string_char))),
        |s| (*s).to_owned(),
    )(i)
}

/// *source_character* **but not** ( *alpha_numeric_character* | *line_terminator* )
pub(crate) fn non_escaped_double_quoted_string_char(i: Input) -> StringResult {
    peek(not(alpha_numeric_character))(i)?;
    peek(not(line_terminator))(i)?;
    map(anychar, |c: char| c.to_string())(i)
}

/// `\` `x` *octal_digit* *octal_digit*? *octal_digit*?
pub(crate) fn octal_escape_sequence(i: Input) -> StringResult {
    map(
        recognize(tuple((
            tag("\\x"),
            octal_digit,
            opt(octal_digit),
            opt(octal_digit),
        ))),
        |s| (*s).to_owned(),
    )(i)
}

/// `\` *hexadecimal_digit* *hexadecimal_digit*?
pub(crate) fn hexadecimal_escape_sequence(i: Input) -> StringResult {
    map(
        recognize(tuple((
            char('\\'),
            hexadecimal_digit,
            opt(hexadecimal_digit),
        ))),
        |s| (*s).to_owned(),
    )(i)
}

/// `\` ( `C` `-` | `c` ) *control_escaped_character*
pub(crate) fn control_escape_sequence(i: Input) -> StringResult {
    map(
        recognize(tuple((
            char('\\'),
            alt((tag("C-"), tag("c"))),
            control_escaped_character,
        ))),
        |s: Input| (*s).to_owned(),
    )(i)
}

/// *double_escape_sequence* | `?` | *source_character* **but not** ( `\` | `?` )
pub(crate) fn control_escaped_character(i: Input) -> StringResult {
    map(
        recognize(alt((
            double_escape_sequence,
            map(tag("?"), |s: Input| (*s).to_owned()),
            map(none_of("\\?"), |c: char| c.to_string()),
        ))),
        |s: Input| (*s).to_owned(),
    )(i)
}

/// `#` *global_variable_identifier* | `#` *class_variable_identifier* | `#` *instance_variable_identifier* | `#` `{` *compound_statement* `}`
pub(crate) fn interpolated_character_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// *uppercase_character* | *lowercase_character* | *decimal_digit*
pub(crate) fn alpha_numeric_character(i: Input) -> CharResult {
    verify(anychar, |c: &char| c.is_ascii_alphanumeric())(i)
}

fn stub_string(i: Input) -> StringResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}
