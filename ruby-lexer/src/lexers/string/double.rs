use crate::lexers::numeric::{hexadecimal_digit, octal_digit};
use crate::lexers::program::{line_terminator, line_terminator_escape_sequence};
use crate::{CharResult, Input, ParseResult, StringResult};
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
    // Should be evaluated
    alt((
        map(simple_escape_sequence, |c| c.to_string()),
        map(non_escaped_sequence, |s| (*s).to_owned()),
        line_terminator_escape_sequence,
        octal_escape_sequence,
        hexadecimal_escape_sequence,
        control_escape_sequence,
    ))(i)
}

/// `\` *double_escaped_character*
pub(crate) fn simple_escape_sequence(i: Input) -> CharResult {
    map(tuple((char('\\'), double_escaped_character)), |t| {
        match t.1 {
            '\\' => '\\',
            'n' => '\n',
            't' => '\t',
            'r' => '\r',
            'f' => '\x0c',
            'v' => '\x0b',
            'a' => '\x07',
            'e' => '\x1b',
            'b' => '\x08',
            's' => ' ',
            _ => unreachable!(),
        }
    })(i)
}

/// `\` | `n` | `t` | `r` | `f` | `v` | `a` | `e` | `b` | `s`
pub(crate) fn double_escaped_character(i: Input) -> CharResult {
    one_of("\\ntrfvaebs")(i)
}

/// `\` *non_escaped_double_quoted_string_char*
pub(crate) fn non_escaped_sequence(i: Input) -> ParseResult {
    recognize(tuple((char('\\'), non_escaped_double_quoted_string_char)))(i)
}

/// *source_character* **but not** ( *alpha_numeric_character* | *line_terminator* )
pub(crate) fn non_escaped_double_quoted_string_char(i: Input) -> CharResult {
    peek(not(alpha_numeric_character))(i)?;
    peek(not(line_terminator))(i)?;
    anychar(i)
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
    // Should be evaluated
    stub_string(i)
}

/// *uppercase_character* | *lowercase_character* | *decimal_digit*
pub(crate) fn alpha_numeric_character(i: Input) -> CharResult {
    verify(anychar, |c: &char| c.is_ascii_alphanumeric())(i)
}

fn stub_string(i: Input) -> StringResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_double_quoted_string() {
        use_parser!(double_quoted_string, Input, String, ErrorKind);
        // Parse errors
        // Success cases
    }

    #[test]
    fn test_double_quoted_string_characer() {
        use_parser!(double_quoted_string_character, Input, String, ErrorKind);
        // Parse errors
        // Success cases
    }

    #[test]
    fn test_double_escape_sequence() {
        use_parser!(double_escape_sequence, Input, String, ErrorKind);
        // Parse errors
        // Success cases
    }

    #[test]
    fn test_non_escaped_sequence() {
        use_parser!(non_escaped_sequence, Input, Input => &str, ErrorKind);
        // Parse errors
        // Success cases
    }

    #[test]
    fn test_simple_escape_sequence() {
        use_parser!(simple_escape_sequence, Input, char, ErrorKind);
        // Parse errors
        assert_err!("");
        assert_err!("s");
        assert_err!("v");
        // Success cases
        assert_ok!("\\\\", '\\');
        assert_ok!("\\n", '\n');
        assert_ok!("\\t", '\t');
        assert_ok!("\\r", '\r');
        assert_ok!("\\f", '\x0c');
        assert_ok!("\\v", '\x0b');
        assert_ok!("\\a", '\x07');
        assert_ok!("\\e", '\x1b');
        assert_ok!("\\b", '\x08');
        assert_ok!("\\s", ' ');
    }
}
