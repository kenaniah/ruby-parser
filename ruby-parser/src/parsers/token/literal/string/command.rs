use super::double::double_escape_sequence;
use super::double::interpolated_character_sequence;
use super::quoted::expanded_delimited_string;
use crate::lexer::*;
use crate::parsers::program::source_character;

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
        map(interpolated_character_sequence, |e| Segment::expr(e)),
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
            "%x!foo#@hi [bar] [#{%Q((hello))}]!",
            vec![
                Node::Segment(Segment::String("foo".to_owned())),
                Node::ident("@hi", IdentifierKind::InstanceVariable),
                Node::Segment(Segment::String(" [bar] [".to_owned())),
                Node::Block(vec![Node::literal_string("(hello)")]),
                Node::Segment(Segment::String("]".to_owned())),
            ]
        );
    }
}
