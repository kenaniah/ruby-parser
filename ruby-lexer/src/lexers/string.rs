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
    stub_string(i)
}

/// `\` *single_quoted_string_non_escaped_character*
pub fn single_quoted_string_non_escaped_character_sequence(i: Input) -> StringResult {
    let (i, char1) = char('\\')(i)?;
    let (i, char2) = single_quoted_string_non_escaped_character(i)?;
    let mut string = String::with_capacity(char1.len_utf8() + char2.len_utf8());
    string.push(char1);
    string.push(char2);
    Ok((i, string))
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

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_source_character() {
        use_parser!(source_character, Input, char, ErrorKind);
        // Success cases
        assert_ok!("1", '1');
        assert_ok!("東", '東');
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
        assert_ok!("\\東", "\\東".to_owned());
    }
}
