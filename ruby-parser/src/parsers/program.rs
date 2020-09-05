//! Provides parsers for program text
use crate::lexer::*;
use crate::parsers::comment::comment;
use crate::parsers::statement::statement;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, line_ending, one_of};
use nom::combinator::{map, opt, recognize};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{terminated, tuple};

/// *compound_statement*
pub fn program(i: Input) -> NodeResult {
    compound_statement(i)
}

/// *statement_list*? *separator_list*?
pub(crate) fn compound_statement(i: Input) -> NodeResult {
    map(terminated(opt(statement_list), opt(separator_list)), |cs| {
        cs.unwrap_or(Node::Block(vec![]))
    })(i)
}

/// *statement* ( *separator_list* *statement* )*
pub(crate) fn statement_list(i: Input) -> NodeResult {
    let (i, _) = opt(separator_list)(i)?;
    map(separated_list0(separator_list, statement), |statements| {
        Node::Block(statements)
    })(i)
}

/// *separator*+
pub(crate) fn separator_list(i: Input) -> LexResult {
    recognize(many1(separator))(i)
}

/// `;` | *line_terminator*
pub(crate) fn separator(i: Input) -> LexResult {
    map(
        tuple((
            many0(whitespace),
            alt((line_terminator, recognize(char(';')))),
            many0(whitespace),
        )),
        |t| t.1,
    )(i)
}

/// Any UTF-8 scalar value (a Rust `char`)
pub(crate) fn source_character(i: Input) -> CharResult {
    anychar(i)
}

/// 0x09 | 0x0b | 0x0c | 0x0d | 0x20 | *line_terminator_escape_sequence*
pub(crate) fn whitespace(i: Input) -> LexResult {
    alt((
        recognize(one_of(" \t\x0b\x0c\r")),
        line_terminator_escape_sequence,
    ))(i)
}

/// `\r`? `\n`
pub(crate) fn line_terminator(i: Input) -> LexResult {
    line_ending(i)
}

/// `\` *line_terminator*
pub(crate) fn line_terminator_escape_sequence(i: Input) -> LexResult {
    recognize(tuple((char('\\'), line_terminator)))(i)
}

/// [ beginning of a line ] `__END__` ( *line_terminator* | [ end of a program ] )
pub(crate) fn end_of_program_marker(i: Input) -> NodeResult {
    if !i.beginning_of_line() {
        return Err(nom::Err::Error((i, nom::error::ErrorKind::Space)));
    }
    let (i, _) = tag("__END__")(i)?;
    let (i, _) = opt(line_terminator)(i)?;
    Ok((i, Node::EndOfProgram))
}

/// ( *whitespace* | *line_terminator* | *comment* )*
pub(crate) fn ws(i: Input) -> LexResult {
    recognize(many0(alt((
        whitespace,
        line_terminator,
        recognize(comment),
    ))))(i)
}

/// *whitespace**
pub(crate) fn no_lt(i: Input) -> LexResult {
    recognize(many0(whitespace))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_statement() {
        use_parser!(compound_statement);
        assert_ok!("2; 5", Node::Block(vec![Node::int(2), Node::int(5)]));
    }

    #[test]
    fn test_source_character() {
        use_parser!(source_character);
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
        use_parser!(line_terminator);
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
        use_parser!(line_terminator_escape_sequence);
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
        use_parser!(whitespace);
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

    // #[test]
    // fn test_end_of_program_marker() {
    //     use_parser!(end_of_program_marker);
    //     // Success cases
    //     assert_ok!("__END__");
    //     assert_ok!("__END__\n");
    //     assert_ok!("__END__\r\n");
    //     // Failure cases
    //     assert_err!("__end__");
    //     assert_err!("__END__ing");
    // }
}
