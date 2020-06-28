//! Provides parsers for program text
use crate::{CharResult, Input, ParseResult, StringResult, Token, TokenResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, line_ending, one_of};
use nom::combinator::{map, opt, recognize};
use nom::sequence::tuple;

// /// *compound_statement*
// pub fn program(i: Input) {
//     compound_statement(i)
// }
//
// /// *statement_list*? *separator_list*?
// pub(crate) fn compound_statement(i: Input) {}
//
// /// *statement* ( *separator_list* *statement* )*
// pub(crate) fn statement_list(i: Input){}
//
// /// *separator*+
// pub(crate) fn separator_list(i: Input){}
//
// /// `;` | *line_terminator*
// pub(crate) fn separator(i: Input){}
//
// /// *line_terminator* | *whitespace* | *comment* | *end_of_program_marker* | *token*
// pub(crate) fn input_element(i: Input) {}

/// Any UTF-8 scalar value (a Rust `char`)
pub(crate) fn source_character(i: Input) -> CharResult {
    anychar(i)
}

/// 0x09 | 0x0b | 0x0c | 0x0d | 0x20 | *line_terminator_escape_sequence*
pub(crate) fn whitespace(i: Input) -> ParseResult {
    alt((
        recognize(one_of(" \t\x0b\x0c\r")),
        line_terminator_escape_sequence,
    ))(i)
}

/// `\r`? `\n`
pub(crate) fn line_terminator(i: Input) -> ParseResult {
    line_ending(i)
}

/// `\` *line_terminator*
pub(crate) fn line_terminator_escape_sequence(i: Input) -> ParseResult {
    recognize(tuple((char('\\'), line_terminator)))(i)
}

/// [ beginning of a line ] `__END__` ( *line_terminator* | [ end of a program ] )
pub(crate) fn end_of_program_marker(i: Input) -> TokenResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, nom::error::ErrorKind::Space)));
    }
    let (i, _) = tag("__END__")(i)?;
    let (i, _) = opt(line_terminator)(i)?;
    Ok((i, Token::EndOfProgram))
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
        assert_eq!(
            source_character("é".into()),
            Ok((Input::new_with_pos("\u{301}", 1, 1, 2), 'e'))
        ); // U+0065: 'latin small letter e' + U+0301: 'combining acute accent'
        assert_eq!(
            source_character("é".into()),
            Ok((Input::new_with_pos("", 2, 1, 2), 'é'))
        ); // U+00e9: 'latin small letter e with acute'
        assert_eq!(
            source_character("東".into()),
            Ok((Input::new_with_pos("", 3, 1, 2), '東'))
        ); // U+6771: 'CJK Unified Ideograph-6771' "East"
    }

    #[test]
    fn test_line_terminator() {
        use_parser!(line_terminator, Input, Input, ErrorKind);
        // Success cases
        assert_ok!("\n");
        assert_ok!("\r\n");
        // Failure cases
        assert_err!("");
        assert_err!(" ");
        assert_err!("\r");
    }

    #[test]
    fn test_line_terminator_escape_sequence() {
        use_parser!(line_terminator_escape_sequence, Input, Input, ErrorKind);
        // Success cases
        assert_ok!("\\\n");
        assert_ok!("\\\r\n");
        // Failure cases
        assert_err!("\n");
        assert_err!("\r");
        assert_err!("\r\n");
    }

    #[test]
    fn test_whitespace() {
        use_parser!(whitespace, Input, Input, ErrorKind);
        // Success cases
        assert_ok!(" ");
        assert_ok!("\t");
        assert_ok!("\r");
        assert_ok!("\x0b");
        assert_ok!("\x0c");
        assert_ok!("\\\n");
        assert_ok!("\\\r\n");
        // Failure cases
        assert_err!("\n");
        assert_err!("\r\n");
    }

    #[test]
    fn test_end_of_program_marker() {
        use_parser!(end_of_program_marker, Input, Token, ErrorKind);
        // Success cases
        assert_ok!("__END__");
        assert_ok!("__END__\n");
        assert_ok!("__END__\r\n");
        // Failure cases
        assert_err!("__end__");
        assert_err!("__END__ing");
    }
}
