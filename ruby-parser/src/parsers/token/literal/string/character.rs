use super::double::double_escape_sequence;
use crate::lexer::*;
use crate::parsers::program::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, not, peek};
use nom::sequence::preceded;

/// `?` ( *double_escape_sequence* | *source_character* **but not** ( *whitespace* | `\` ) )
pub(crate) fn character_literal(i: Input) -> StringResult {
    preceded(
        char('?'),
        alt((
            // An escaped newline should not be treated as a line continuation in this context
            map(tag("\\\n"), |_| "\n".to_owned()),
            double_escape_sequence,
            map(
                preceded(peek(not(alt((whitespace, tag("\\"))))), source_character),
                |c| c.to_string(),
            ),
        )),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_literal() {
        use_parser!(character_literal);
        // Parse errors
        assert_err!("");
        assert_err!("?");
        assert_err!("? ");
        assert_err!("?\\");
        assert_err!("?\t");
        assert_err!("a");
        assert_err!("?ab");
        assert_err!("?a ");
        assert_err!("?\\u");
        assert_err!("?1234");
        // Success cases
        assert_ok!("?2", "2");
        assert_ok!("?a", "a");
        assert_ok!("?:", ":");
        assert_ok!("?😄", "😄");
        assert_ok!("?東", "東"); // U+6771: 'CJK Unified Ideograph-6771' "East"
        assert_ok!("?\\k", "k");
        assert_ok!("?\\ ", " ");
        assert_ok!("?\\\\", "\\");
        assert_ok!("?\\n", "\n");
        assert_ok!("?\\\n", "\n");
        assert_ok!("?\\\t", "\t");
        assert_ok!("?\\123", "S");
        assert_ok!("?\\u{0000 0002}", "\u{0}\u{2}");
    }
}
