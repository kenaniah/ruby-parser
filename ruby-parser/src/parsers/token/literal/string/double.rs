use crate::lexer::*;
use crate::parsers::program::*;
use crate::parsers::token::identifier::*;
use crate::parsers::token::literal::numeric::{hexadecimal_digit, octal_digit};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, none_of, one_of};
use nom::combinator::{map, not, peek, recognize};
use nom::multi::{many0, many1, many_m_n, separated_list1};
use nom::sequence::{preceded, tuple};
use std::convert::TryFrom;

/// `"` *double_quoted_string_character** `"`
pub(crate) fn double_quoted_string(i: Input) -> InterpolatableResult {
    let (i, _) = char('"')(i)?;
    let (i, contents) = many0(double_quoted_string_character)(i)?;
    let (i, _) = char('"')(i)?;

    Ok((i, Interpolatable::from(contents)))
}

/// *source_character* **but not** ( `"` | `#` | `\` ) | `#` **not** ( `$` | `@` | `{` ) | *double_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn double_quoted_string_character(i: Input) -> SegmentResult {
    alt((
        map(none_of("\"#\\"), |c| Segment::Char(c)),
        map(double_escape_sequence, |s| Segment::String(s)),
        map(interpolated_character_sequence, |e| Segment::expr(e)),
        map(char('#'), |c| Segment::Char(c)),
    ))(i)
}

/// *simple_escape_sequence* | *non_escaped_sequence* | *line_terminator_escape_sequence* | *octal_escape_sequence* | *hexadecimal_escape_sequence* | *control_escape_sequence*
pub(crate) fn double_escape_sequence(i: Input) -> StringResult {
    let i = stack_frame!("double_escape_sequence", i);
    // Should be evaluated
    alt((
        map(simple_escape_sequence, |c| c.to_string()),
        map(line_terminator_escape_sequence, |_s| String::new()),
        map(octal_escape_sequence, |c| c.to_string()),
        map(hexadecimal_escape_sequence, |c| c.to_string()),
        map(single_unicode_escape_sequence, |c| c.to_string()),
        multiple_unicode_escape_sequence,
        control_escape_sequence,
        map(non_escaped_sequence, |c| c.to_string()),
    ))(i)
}

/// `\` *double_escaped_character*
pub(crate) fn simple_escape_sequence(i: Input) -> CharResult {
    let i = stack_frame!("simple_escape_sequence", i);
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
pub(crate) fn non_escaped_sequence(i: Input) -> CharResult {
    preceded(char('\\'), non_escaped_double_quoted_string_char)(i)
}

/// *source_character* **but not** ( [ any escaping character ] | *line_terminator* )
pub(crate) fn non_escaped_double_quoted_string_char(i: Input) -> CharResult {
    let i = stack_frame!("non_escaped_double_quoted_string_char", i);
    peek(not(one_of("\\ntrfvaebsxucCM01234567")))(i.clone())?;
    peek(not(line_terminator))(i.clone())?;
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
pub(crate) fn unicode_hex_digits(i: Input) -> LexResult {
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
    let chr: u8 = escape.chars().next().unwrap() as u8;
    match (ctrl, meta, chr) {
        (true, false, 0x3F) => Ok((i, "\x7F".to_owned())),
        (true, false, _) => Ok((
            i,
            ((if chr < 0x20 { chr } else { chr & 0x9F }) as char).to_string(),
        )),
        (true, true, _) => Ok((
            i,
            (((if chr < 0x20 { chr } else { chr & 0x9F }) | 0x80) as char).to_string(),
        )),
        (false, true, _) => Ok((i, (((chr & 0xFF) | 0x80) as char).to_string())),
        (false, false, _) => unreachable!(),
    }
}

/// *double_escape_sequence* | `?` | *source_character* **but not** ( `\` | `?` )
pub(crate) fn control_escaped_character(i: Input) -> StringResult {
    alt((
        double_escape_sequence,
        map(tag("?"), |s: Input| (*s).to_owned()),
        map(none_of("\\?"), |c: char| c.to_string()),
    ))(i)
}

/// `#` *global_variable_identifier* | `#` *class_variable_identifier* | `#` *instance_variable_identifier* | `#` `{` *compound_statement* `}`
pub(crate) fn interpolated_character_sequence(i: Input) -> NodeResult {
    alt((
        preceded(char('#'), global_variable_identifier),
        preceded(char('#'), class_variable_identifier),
        preceded(char('#'), instance_variable_identifier),
        map(tuple((tag("#{"), compound_statement, char('}'))), |t| t.1),
    ))(i)
}

// Converts the value of an escape sequence into a character
fn char_from_radix(i: &str, radix: u32) -> char {
    char::try_from(u32::from_str_radix(i, radix).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Literal;

    macro_rules! assert_result {
        ($a:expr, $b:expr) => {
            assert_ok!($a, $b.to_owned())
        };
    }

    #[test]
    fn test_double_quoted_string() {
        use_parser!(double_quoted_string);
        fn ds(i: &str) -> Interpolatable {
            Interpolatable::String(i.to_owned())
        }
        fn seg(i: &str) -> Node {
            Node::Segment(Segment::String(i.to_owned()))
        }
        fn is(i: Vec<Node>) -> Interpolatable {
            Interpolatable::Interpolated(i)
        }
        // Parse errors
        assert_err!("''");
        assert_err!("\"");
        assert_err!("#{foo#bar}");
        // Success cases
        assert_ok!("\"\"", ds(""));
        assert_ok!("\"foo\\\nbar\"", ds("foobar"));
        assert_ok!(
            "\"some #thing\\n#$hi\"",
            is(vec![
                seg("some #thing\n"),
                Node::ident("$hi", IdentifierKind::GlobalVariable)
            ])
        );
        assert_ok!(
            "\"#@@VAR#{2; 3.5} \"",
            is(vec![
                Node::ident("@@VAR", IdentifierKind::ClassVariable),
                Node::Block(vec![Node::integer(2), Node::float(3.5)]),
                seg(" ")
            ])
        );
    }

    #[test]
    fn test_double_quoted_string_characer() {
        use_parser!(double_quoted_string_character);
        // Parse errors
        assert_err!("\\");
        assert_err!("\"");
        assert_err!("#{");
        assert_err!("#{\"foo#{2}bar\"");
        // Success cases
        assert_ok!("😀", Segment::Char('😀'));
        assert_ok!("A", Segment::Char('A'));
        assert_ok!("#", Segment::Char('#'));
        assert_ok!("\\\"", Segment::String("\"".to_owned()));
        assert_ok!("\\u0000", Segment::String("\0".to_owned()));
        assert_ok!("#{}", Segment::expr(Node::Block(vec![])));
        assert_ok!(
            "#@@foo",
            Segment::expr(Node::ident("@@foo", IdentifierKind::ClassVariable))
        );
        assert_ok!(
            "#@inst",
            Segment::expr(Node::ident("@inst", IdentifierKind::InstanceVariable))
        );
        assert_ok!(
            "#$glob",
            Segment::expr(Node::ident("$glob", IdentifierKind::GlobalVariable))
        );
        assert_ok!(
            "#{foobar}",
            Segment::expr(Node::Block(vec![Node::ident(
                "foobar",
                IdentifierKind::LocalVariable
            )]))
        );
        assert_ok!(
            "#{\"foo#{2bar\"}",
            Segment::expr(Node::Block(vec![Node::literal_string("foo#{2bar")]))
        );
        assert_ok!(
            "#{\"foo#{2}bar\"}",
            Segment::expr(Node::Block(vec![Node::Interpolated(Interpolated::String(
                vec![
                    Node::Segment(Segment::String("foo".to_owned())),
                    Node::Block(vec![Node::Literal(Literal::Integer(2))]),
                    Node::Segment(Segment::String("bar".to_owned()))
                ]
            ))]))
        );
    }

    #[test]
    fn test_double_escape_sequence() {
        use_parser!(double_escape_sequence);
        // Parse errors
        assert_err!("v");
        assert_err!("\\");
        assert_err!("\r");
        // Success cases
        assert_result!("\\ ", " ");
        assert_result!("\\\\", "\\");
        assert_result!("\\\n", "");
        assert_result!("\\000", "\0");
        assert_result!("\\x7", "\u{07}");
        assert_result!("\\r", "\r");
        assert_result!("\\z", "z");
        assert_result!("\\M-B", "\u{C2}");
        assert_result!("\\uaBcD", "\u{ABCD}");
        assert_result!("\\u{1234 aBCD}", "\u{1234}\u{ABCD}");
    }

    #[test]
    fn test_non_escaped_sequence() {
        use_parser!(non_escaped_sequence);
        // Parse errors
        assert_err!("\\");
        assert_err!("\\\\");
        assert_err!("\\n");
        assert_err!("\\r");
        assert_err!("\\v");
        assert_err!("\\c");
        assert_err!("\\C");
        assert_err!("\\M");
        assert_err!("\\\n");
        assert_err!("\\0");
        assert_err!("\\7");
        // Success cases
        assert_ok!("\\A", 'A');
        assert_ok!("\\😀", '😀');
        assert_ok!("\\z", 'z');
        assert_ok!("\\N", 'N');
        assert_ok!("\\8", '8');
        assert_ok!("\\9", '9');
    }

    #[test]
    fn test_simple_escape_sequence() {
        use_parser!(simple_escape_sequence);
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
        use_parser!(octal_escape_sequence);
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
        use_parser!(hexadecimal_escape_sequence);
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
        use_parser!(single_unicode_escape_sequence);
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
        use_parser!(multiple_unicode_escape_sequence);
        // Parse errors
        assert_err!("\\u");
        assert_err!("\\u1234");
        assert_err!("\\u{123}");
        assert_err!("\\u{12345}");
        assert_err!("\\u{1234 12}");
        assert_err!("\\u{FFFG}");
        // Success cases
        assert_result!("\\u{0000}", "\0");
        assert_result!("\\u{0020}", " ");
        assert_result!("\\u{1234 aBCD}", "\u{1234}\u{ABCD}");
        assert_result!("\\u{  aBcD }", "\u{ABCD}");
        assert_result!("\\u{7FFF   ffff   000A }", "\u{7FFF}\u{FFFF}\n");
    }

    #[test]
    fn test_control_escape_sequence() {
        use_parser!(control_escape_sequence);
        // Parse errors
        assert_err!("\\c");
        assert_err!("\\C");
        assert_err!("\\C-");
        assert_err!("\\C-\\M-");
        assert_err!("\\c-a");
        assert_err!("a");
        // Success cases
        assert_result!("\\C- ", "\0");
        assert_result!("\\C-5", "\u{15}");
        assert_result!("\\cA", "\u{01}");
        assert_result!("\\C-A", "\u{01}");
        assert_result!("\\M- ", "\u{A0}");
        assert_result!("\\M-b", "\u{E2}");
        assert_result!("\\M-B", "\u{C2}");
        assert_result!("\\M-\\C-c", "\u{83}");
        assert_result!("\\M-\\C-C", "\u{83}");
        assert_result!("\\c\\M-D", "\u{84}");
        assert_result!("\\M-?", "\u{BF}");
        assert_result!("\\M-\\C- ", "\u{80}");
        assert_result!("\\M-\\C-?", "\u{9F}");
        assert_result!("\\c?", "\u{7F}");
        assert_result!("\\C-?", "\u{7F}");
        // Multibytes should only look at the first byte
        assert_result!("\\M-\\C-東", "\u{91}");
        assert_result!("\\M-😅", "\u{85}");
        // Escape sequences
        assert_result!("\\C-\\\\", "\u{1C}");
        assert_result!("\\C-\\M-\\\\", "\u{9C}");
        assert_result!("\\C-\n", "\u{0A}");
        assert_result!("\\C-\\n", "\u{0A}");
        assert_result!("\\M-\\C-\\n", "\u{8A}");
        assert_result!("\\C-\\t", "\t");
        assert_result!("\\C-\\z", "\u{1A}");
        assert_result!("\\C-\\C-\\n", "\u{0A}");
    }
}
