use crate::lexers::program::source_character;
use crate::lexers::string::double::double_escape_sequence;
use crate::lexers::string::double::interpolated_character_sequence;
use crate::lexers::string::quoted::expanded_delimited_string;
use crate::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, none_of};
use nom::combinator::{map, peek};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, terminated};

/// *backquoted_external_command_execution* | *quoted_external_command_execution*
pub(crate) fn external_command_execution(i: Input) -> InterpolatableResult {
    alt((
        backquoted_external_command_execution,
        quoted_external_command_execution,
    ))(i)
}

/// ``` *backquoted_external_command_execution_character** ```
pub(crate) fn backquoted_external_command_execution(i: Input) -> InterpolatableResult {
    map(
        delimited(
            char('`'),
            many0(backquoted_external_command_execution_character),
            char('`'),
        ),
        |vecs| Interpolatable::from(vecs),
    )(i)
}

/// *source_character* **but not** ( ``` | `#` | `\` ) | `#` **not** ( `$` | `@` | `{` ) | *double_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn backquoted_external_command_execution_character(i: Input) -> SegmentResult {
    alt((
        map(preceded(peek(none_of("`#\\")), source_character), |c| {
            Segment::Char(c)
        }),
        map(double_escape_sequence, |s| Segment::String(s)),
        map(interpolated_character_sequence, |e| Segment::Expr(e)),
        map(terminated(char('#'), peek(none_of("$@{"))), |c| {
            Segment::Char(c)
        }),
    ))(i)
}

/// `%x` *expanded_delimited_string*
pub(crate) fn quoted_external_command_execution(i: Input) -> InterpolatableResult {
    preceded(tag("%x"), expanded_delimited_string)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_string {
        ($a:expr, $b:expr) => {
            assert_ok!($a, Interpolatable::String($b.to_owned()))
        };
    }
    macro_rules! assert_interpolated {
        ($a:expr, $b:expr) => {
            assert_ok!($a, Interpolatable::Interpolated($b))
        };
    }

    #[test]
    fn test_external_command_execution() {
        use_parser!(external_command_execution);
        // Parse errors
        assert_err!("`");
        assert_err!("`foo");
        assert_err!("%x(foo");
        assert_err!("%x(#{foo)");
        // Success cases
        assert_string!("%x:ls:", "ls");
        assert_string!("%x<foo \\<bar>", "foo <bar");
        assert_string!("`#\\\\foo\\1\\`2`", "#\\foo\u{1}`2");
        assert_interpolated!(
            "%x[foo#@hi [bar] [#{%Q((hello))}]]",
            vec![
                Token::Segment("foo".to_owned()),
                Token::InstanceVariableIdentifier("@hi".to_owned()),
                Token::Segment(" [bar] [".to_owned()),
                Token::Block(vec![Token::String("(hello)".to_owned())]),
                Token::Segment("]".to_owned()),
            ]
        );
    }

}
