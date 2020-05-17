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
    stub_string(i)
}

/// *single_quoted_string_non_escaped_character* | *single_quoted_escape_sequence*
pub fn single_quoted_string_character(i: Input) -> StringResult {
    stub_string(i)
}

/// *single_escape_character_sequence* | *single_quoted_string_non_escaped_character_sequence*
pub fn single_quoted_escape_sequence(i: Input) -> StringResult {
    stub_string(i)
}

/// `\` *single_quoted_string_meta_character*
pub fn single_escape_character_sequence(i: Input) -> StringResult {
    let (i, char1) = char('\\')(i)?;
    let (i, char2) = single_quoted_string_meta_character(i)?;
    Ok((i, string_from_2_chars(char1, char2)))
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

/// Any UTF-8 character
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
        use_parser!(single_quoted_string_non_escaped_character_sequence, Input, String, ErrorKind);
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
        assert_ok!("\\'", "\\'".to_owned());
        assert_ok!("\\\\", "\\\\".to_owned());
    }

}
