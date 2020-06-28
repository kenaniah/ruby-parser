use crate::lexers::numeric::{hexadecimal_digit, octal_digit};
use crate::lexers::program::{line_terminator, line_terminator_escape_sequence};
use crate::{CharResult, Input, ParseResult, StringResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{map, not, opt, peek, recognize, verify};
use nom::multi::{many0, many1, many_m_n, separated_list1};
use nom::sequence::tuple;
use std::convert::TryFrom;

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
        map(line_terminator_escape_sequence, |_s| String::new()),
        map(octal_escape_sequence, |c| c.to_string()),
        map(hexadecimal_escape_sequence, |c| c.to_string()),
        map(single_unicode_escape_sequence, |c| c.to_string()),
        multiple_unicode_escape_sequence,
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

/// `\` *octal_digit* *octal_digit*? *octal_digit*?
pub(crate) fn octal_escape_sequence(i: Input) -> CharResult {
    map(
        tuple((char('\\'), recognize(many_m_n(1, 3, octal_digit)))),
        |t| char_from_radix(*t.1, 8),
    )(i)
}

/// `\` `x` *hexadecimal_digit* *hexadecimal_digit*?
pub(crate) fn hexadecimal_escape_sequence(i: Input) -> CharResult {
    map(
        tuple((tag("\\x"), recognize(many_m_n(1, 2, hexadecimal_digit)))),
        |t| char_from_radix(*t.1, 16),
    )(i)
}

/// `\u` *unicode_hex_digits*
pub(crate) fn single_unicode_escape_sequence(i: Input) -> CharResult {
    map(tuple((tag("\\u"), unicode_hex_digits)), |t| {
        char_from_radix(*t.1, 16)
    })(i)
}

/// `\u{` 0x20* *unicode_hex_digits* ( 0x20+ *unicode_hex_digits* )* 0x20* `}`
pub(crate) fn multiple_unicode_escape_sequence(i: Input) -> StringResult {
    map(
        tuple((
            tag("\\u{"),
            many0(char(' ')),
            separated_list1(many1(char(' ')), unicode_hex_digits),
            many0(char(' ')),
            tag("}"),
        )),
        |t| {
            let mut str = String::new();
            for chr in t.2 {
                str.push(char_from_radix(*chr, 16));
            }
            str
        },
    )(i)
}

/// *hexadecimal_digit* *hexadecimal_digit* *hexadecimal_digit* *hexadecimal_digit*
pub(crate) fn unicode_hex_digits(i: Input) -> ParseResult {
    recognize(many_m_n(4, 4, hexadecimal_digit))(i)
}

/// ( `\` ( `C-` | `c` ) **and/or** `\M-` ) *control_escaped_character*
pub(crate) fn control_escape_sequence(i: Input) -> StringResult {
    let (i, ((ctrl, meta), escape)) = tuple((
        alt((
            map(
                alt((
                    tag("\\C-\\M-"),
                    tag("\\c\\M-"),
                    tag("\\M-\\C-"),
                    tag("\\M-\\c"),
                )),
                |_| (true, true),
            ),
            map(alt((tag("\\C-"), tag("\\c"))), |_| (true, false)),
            map(tag("\\M-"), |_| (false, true)),
        )),
        control_escaped_character,
    ))(i)?;
    println!("ctrl: {:?}, meta: {:?}, escape: {:?}", ctrl, meta, escape);
    match (ctrl, meta, &escape[..]) {
        (true, false, "?") => Ok((i, "\x7F".to_owned())),
        (true, false, _) => Ok((i, "ctrl".to_owned())),
        (true, true, _) => Ok((i, "ctrl+meta".to_owned())),
        (false, true, _) => Ok((i, "meta".to_owned())),
        (false, false, _) => unreachable!(),
    }
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

// Converts the value of an escape sequence into a character
fn char_from_radix(i: &str, radix: u32) -> char {
    char::try_from(u32::from_str_radix(i, radix).unwrap()).unwrap()
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
        assert_ok!("\\f", '\x0C');
        assert_ok!("\\v", '\x0B');
        assert_ok!("\\a", '\x07');
        assert_ok!("\\e", '\x1B');
        assert_ok!("\\b", '\x08');
        assert_ok!("\\s", ' ');
    }

    #[test]
    fn test_octal_escape_sequence() {
        use_parser!(octal_escape_sequence, Input, char, ErrorKind);
        // Parse errors
        assert_err!("\\");
        assert_err!("\\9");
        assert_err!("\\0a");
        assert_err!("\\9");
        assert_err!("\\1234");
        assert_err!("\\x0");
        // Success cases
        assert_ok!("\\0", '\0');
        assert_ok!("\\000", '\0');
        assert_ok!("\\7", '\u{7}');
        assert_ok!("\\40", ' ');
        assert_ok!("\\040", ' ');
        assert_ok!("\\77", '?');
        assert_ok!("\\150", 'h');
        assert_ok!("\\374", '\u{FC}');
        assert_ok!("\\776", '\u{1FE}'); // MRI truncates to just the last byte (\xFE)
    }

    #[test]
    fn test_hexadecimal_escape_sequence() {
        use_parser!(hexadecimal_escape_sequence, Input, char, ErrorKind);
        // Parse errors
        assert_err!("\\");
        assert_err!("\\x");
        assert_err!("\\xh");
        assert_err!("\\xFFa");
        assert_err!("\\XFF");
        // Success cases
        assert_ok!("\\x0", '\0');
        assert_ok!("\\x00", '\0');
        assert_ok!("\\x7", '\u{07}');
        assert_ok!("\\x20", ' ');
        assert_ok!("\\x0A", '\n');
        assert_ok!("\\x36", '6');
        assert_ok!("\\x72", '\x72');
        assert_ok!("\\xfa", '\u{FA}');
        assert_ok!("\\xFF", '\u{FF}');
    }

    #[test]
    fn test_single_unicode_escape_sequence() {
        use_parser!(single_unicode_escape_sequence, Input, char, ErrorKind);
        // Parse errors
        assert_err!("\\u");
        assert_err!("\\u123");
        assert_err!("\\u12345");
        assert_err!("\\uFFFG");
        // Success cases
        assert_ok!("\\u0000", '\0');
        assert_ok!("\\u0020", ' ');
        assert_ok!("\\u1234", '\u{1234}');
        assert_ok!("\\uaBcD", '\u{ABCD}');
        assert_ok!("\\u7FFF", '\u{7FFF}');
        assert_ok!("\\uFFFF", '\u{FFFF}');
    }

    #[test]
    fn test_multiple_unicode_escape_sequence() {
        use_parser!(multiple_unicode_escape_sequence, Input, String, ErrorKind);
        // Parse errors
        assert_err!("\\u");
        assert_err!("\\u1234");
        assert_err!("\\u{123}");
        assert_err!("\\u{12345}");
        assert_err!("\\u{1234 12}");
        assert_err!("\\u{FFFG}");
        // Success cases
        assert_ok!("\\u{0000}", "\0".to_owned());
        assert_ok!("\\u{0020}", " ".to_owned());
        assert_ok!("\\u{1234 aBCD}", "\u{1234}\u{ABCD}".to_owned());
        assert_ok!("\\u{  aBcD }", "\u{ABCD}".to_owned());
        assert_ok!("\\u{7FFF   ffff   000A }", "\u{7FFF}\u{FFFF}\n".to_owned());
    }

    #[test]
    fn test_control_escape_sequence() {
        use_parser!(control_escape_sequence, Input, String, ErrorKind);
        // Parse errors
        assert_err!("\\c");
        assert_err!("\\C");
        assert_err!("\\C-");
        assert_err!("\\C-\\M-");
        assert_err!("\\c-a");
        assert_err!("a");
        // Success cases
        assert_ok!("\\cA");
        assert_ok!("\\C-A");
        assert_ok!("\\M-B");
        assert_ok!("\\M-\\C-C");
        assert_ok!("\\c\\M-D");
        assert_ok!("\\c?", "\x7F".to_owned());
        assert_ok!("\\C-?", "\x7F".to_owned());
    }
}
