/*!
Provides support for lexing Ruby's string literal formats.
!*/

use nom::branch::alt;
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{map, opt, value};
use nom::multi::many0;
use nom::sequence::{preceded, tuple};

use crate::{CharResult, Input, StringResult, Token, TokenResult};

/// `'` *single_quoted_string_character** `'`
pub fn single_quoted_string(i: Input) -> StringResult {
    let (i, _) = char('\'')(i)?;
    let (i, contents) = many0(single_quoted_string_character)(i)?;
    let (i, _) = char('\'')(i)?;
    let mut string = String::new();
    for s in contents {
        string.push_str(&s);
    }
    Ok((i, string))
}

/// *single_quoted_string_non_escaped_character* | *single_quoted_escape_sequence*
pub fn single_quoted_string_character(i: Input) -> StringResult {
    alt((
        map(single_quoted_string_non_escaped_character, |char| {
            char.to_string()
        }),
        single_quoted_escape_sequence,
    ))(i)
}

/// *single_escape_character_sequence* | *single_quoted_string_non_escaped_character_sequence*
pub fn single_quoted_escape_sequence(i: Input) -> StringResult {
    alt((
        single_escape_character_sequence,
        single_quoted_string_non_escaped_character_sequence,
    ))(i)
}

/// `\` *single_quoted_string_meta_character*
pub fn single_escape_character_sequence(i: Input) -> StringResult {
    let (i, _) = char('\\')(i)?;
    let (i, char) = single_quoted_string_meta_character(i)?;
    Ok((i, char.to_string()))
}

/// `\` *single_quoted_string_non_escaped_character*
pub fn single_quoted_string_non_escaped_character_sequence(i: Input) -> StringResult {
    let (i, char1) = char('\\')(i)?;
    let (i, char2) = single_quoted_string_non_escaped_character(i)?;
    Ok((i, string_from_2_chars(char1, char2)))
}

/// `'` | `\`
pub fn single_quoted_string_meta_character(i: Input) -> CharResult {
    one_of("'\\")(i)
}

/// *source_character* **but not** *single_quoted_string_meta_character*
pub fn single_quoted_string_non_escaped_character(i: Input) -> CharResult {
    none_of("'\\")(i)
}

/// `"` *double_quoted_string_character** `"`
pub fn double_quoted_string(i: Input) -> StringResult {
    stub_string(i)
}

/// *source_character* **but not** ( `"` | `#` | `\` ) | `#` **not** ( `$` | `@` | `{` ) | *double_escape_sequence* | *interpolated_character_sequence*
pub fn double_quoted_string_character(i: Input) -> StringResult {
    stub_string(i)
}

/// *simple_escape_sequence* | *non_escaped_sequence* | *line_terminator_escape_sequence* | *octal_escape_sequence* | *hexadecimal_escape_sequence* | *control_escape_sequence*
pub fn double_escape_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// `\` *double_escaped_character*
pub fn simple_escape_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// `\` | `n` | `t` | `r` | `f` | `v` | `a` | `e` | `b` | `s`
pub fn double_escaped_character(i: Input) -> CharResult {
    one_of("\\ntrfvaebs")(i)
}

/// `\` *non_escaped_double_quoted_string_char*
pub fn non_escaped_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// *source_character* **but not** ( *alpha_numeric_character* | *line_terminator* )
pub fn non_escaped_double_quoted_string_char(i: Input) -> StringResult {
    stub_string(i)
}

/// `\` `x` *octal_digit* *octal_digit*? *octal_digit*?
pub fn octal_escape_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// `\` *hexadecimal_digit* *hexadecimal_digit*?
pub fn hexadecimal_escape_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// `\` ( `C` `-` | `c` ) *control_escaped_character*
pub fn control_escape_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// *double_escape_sequence* | `?` | *source_character* **but not** ( `\` | `?` )
pub fn control_escaped_character(i: Input) -> StringResult {
    stub_string(i)
}

/// `#` *global_variable_identifier* | `#` *class_variable_identifier* | `#` *instance_variable_identifier* | `#` `{` *computed_statement* `}`
pub fn interpolated_character_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// *uppercase_character* | *lowercase_character* | *decimal_digit*
pub fn alpha_numeric_character(i: Input) -> StringResult {
    stub_string(i)
}

/// Any UTF-8 scalar value (a Rust `char`)
pub fn source_character(i: Input) -> CharResult {
    anychar(i)
}

fn stub_token(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}

fn stub_string(i: Input) -> StringResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}

fn stub_char(i: Input) -> CharResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}

/// Constructs a string from two characters
fn string_from_2_chars(c1: char, c2: char) -> String {
    let mut string = String::with_capacity(c1.len_utf8() + c2.len_utf8());
    string.push(c1);
    string.push(c2);
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_source_character() {
        use_parser!(source_character, Input, char, ErrorKind);
        // Success cases
        assert_ok!("1", '1');
        assert_ok!("é", 'é'); // U+00e9: 'latin small letter e with acute'
        assert_ok!("東", '東'); // U+6771: 'CJK Unified Ideograph-6771' "East"
                                // Combined characters
        assert_eq!(source_character("é"), Ok(("\u{301}", 'e'))); // U+0065: 'latin small letter e' + U+0301: 'combining acute accent'
    }

    #[test]
    fn test_single_quoted_string_non_escaped_character_sequence() {
        use_parser!(
            single_quoted_string_non_escaped_character_sequence,
            Input,
            String,
            ErrorKind
        );
        // Parse errors
        assert_err!("");
        assert_err!("\\");
        assert_err!("\\\\");
        assert_err!("\\'");
        assert_err!("foo");
        // Success cases
        assert_ok!("\\1", "\\1".to_owned());
        assert_ok!("\\a", "\\a".to_owned());
        assert_ok!("\\東", "\\東".to_owned()); // U+6771: 'CJK Unified Ideograph-6771' "East"
    }

    #[test]
    fn test_single_escape_character_sequence() {
        use_parser!(single_escape_character_sequence, Input, String, ErrorKind);
        // Parse errors
        assert_err!("");
        assert_err!("\\");
        assert_err!("\\1");
        assert_err!("\\a");
        assert_err!("foo");
        // Success cases
        assert_ok!("\\'", "'".to_owned());
        assert_ok!("\\\\", "\\".to_owned());
    }

    #[test]
    fn test_single_quoted_string() {
        use_parser!(single_quoted_string, Input, String, ErrorKind);
        // Parse errors
        assert_err!("");
        assert_err!("\\''");
        assert_err!("'\\\\''");
        assert_err!("foo");
        assert_err!("'");
        assert_err!("'''");
        assert_err!("'foo");
        // Success cases
        assert_ok!("''");
        assert_ok!("'\\''");
        assert_ok!("'This is a normal string.'", "This is a normal string.".to_owned());
        assert_ok!("'Here\\'s \\a \"handful\" of chars: \\\\ \n \0 東 é é.'", "Here's \\a \"handful\" of chars: \\ \n \0 東 é é.".to_owned());
        assert_ok!("'\\a\\'\\\\'", "\\a'\\".to_owned());
    }
}
