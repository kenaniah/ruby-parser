//! Provides parsers for program text

use crate::{CharResult, Input, StringResult};
use nom::character::complete::{anychar, char, line_ending, one_of};
use nom::combinator::map;

/// *line-terminator* | *whitespace* | *comment* | *end_of_program_marker* | *token*
pub(crate) fn input_element() {}

/// Any UTF-8 scalar value (a Rust `char`)
pub(crate) fn source_character(i: Input) -> CharResult {
    anychar(i)
}

/// 0x09 | 0x0b | 0x0c | 0x0d | 0x20 | *line_terminator_escape_sequence*
pub(crate) fn whitespace(i: Input) -> CharResult {
    //' ' | '\t' | '\x0b' | '\x0c' | '\r'
    one_of(" \t\x0b\x0c\r")(i)
}

/// `\r`? `\n`
pub(crate) fn line_terminator(i: Input) -> StringResult {
    map(line_ending, |s: Input| (*s.input()).to_owned())(i)
}

/// `\` *line_terminator*
pub(crate) fn line_terminator_escape_sequence(i: Input) -> StringResult {
    let (i, char) = char('\\')(i)?;
    let (i, mut string) = line_terminator(i)?;
    string.insert(0, char);
    Ok((i, string))
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
        assert_eq!(source_character("é".into()), Ok(("\u{301}".into(), 'e'))); // U+0065: 'latin small letter e' + U+0301: 'combining acute accent'
    }
}
