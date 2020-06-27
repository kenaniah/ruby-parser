use crate::{CharResult, Input, StringResult};
use nom::branch::alt;
use nom::character::complete::{char, none_of, one_of};
use nom::combinator::map;
use nom::multi::many0;

/// `'` *single_quoted_string_character** `'`
pub(crate) fn single_quoted_string(i: Input) -> StringResult {
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
pub(crate) fn single_quoted_string_character(i: Input) -> StringResult {
    alt((
        map(single_quoted_string_non_escaped_character, |char| {
            char.to_string()
        }),
        single_quoted_escape_sequence,
    ))(i)
}

/// *single_escape_character_sequence* | *single_quoted_string_non_escaped_character_sequence*
pub(crate) fn single_quoted_escape_sequence(i: Input) -> StringResult {
    alt((
        single_escape_character_sequence,
        single_quoted_string_non_escaped_character_sequence,
    ))(i)
}

/// `\` *single_quoted_string_meta_character*
pub(crate) fn single_escape_character_sequence(i: Input) -> StringResult {
    let (i, _) = char('\\')(i)?;
    let (i, char) = single_quoted_string_meta_character(i)?;
    Ok((i, char.to_string()))
}

/// `\` *single_quoted_string_non_escaped_character*
pub(crate) fn single_quoted_string_non_escaped_character_sequence(i: Input) -> StringResult {
    let (i, char1) = char('\\')(i)?;
    let (i, char2) = single_quoted_string_non_escaped_character(i)?;
    Ok((i, string_from_2_chars(char1, char2)))
}

/// `'` | `\`
pub(crate) fn single_quoted_string_meta_character(i: Input) -> CharResult {
    one_of("'\\")(i)
}

/// *source_character* **but not** *single_quoted_string_meta_character*
pub(crate) fn single_quoted_string_non_escaped_character(i: Input) -> CharResult {
    none_of("'\\")(i)
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
        assert_ok!(
            "'This is a normal string.'",
            "This is a normal string.".to_owned()
        );
        assert_ok!(
            "'Here\\'s \\a \"handful\" of chars: \\\\ \n \0 東 é é.'",
            "Here's \\a \"handful\" of chars: \\ \n \0 東 é é.".to_owned()
        );
        // Semantics
        assert_ok!("'\\a\\'\\\\'", "\\a'\\".to_owned());
        // Positioning
        assert_eq!(
            single_quoted_string("'One\nTwo\nThree'".into()),
            Ok((
                Input::new_with_pos("", 15, 3, 7),
                "One\nTwo\nThree".to_owned()
            ))
        );
        assert_eq!(
            single_quoted_string("''".into()),
            Ok((Input::new_with_pos("", 2, 1, 3), "".to_owned()))
        );
        assert_eq!(
            single_quoted_string("'\n'".into()),
            Ok((Input::new_with_pos("", 3, 2, 2), "\n".to_owned()))
        );
    }
}
