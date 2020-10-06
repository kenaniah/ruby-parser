use crate::lexer::*;
use crate::parsers::program::line_terminator_escape_sequence;
use crate::parsers::token::literal::string::double::interpolated_character_sequence;
use crate::parsers::token::literal::string::quoted::expanded_literal_string;
use crate::parsers::token::literal::string::quoted::literal_beginning_delimiter;
use crate::parsers::token::literal::string::quoted::literal_ending_delimiter;

/// `/` *regular_expression_body* `/` *regular_expression_option** | `%r` *literal_beginning_delimiter* *expanded_literal_string** *literal_ending_delimiter* *regular_expression_option**
pub(crate) fn regular_expression_literal(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                char('/'),
                regular_expression_body,
                char('/'),
                many0(regular_expression_option),
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                tag("%r"),
                literal_beginning_delimiter,
                many0(expanded_literal_string),
                literal_ending_delimiter,
                many0(regular_expression_option),
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *regular_expression_character**
pub(crate) fn regular_expression_body(i: Input) -> InterpolatableResult {
    map(many0(regular_expression_character), |contents| {
        Interpolatable::from(contents)
    })(i)
}

/// *source_character* **but not** ( `/` | `#` | `\` ) | `#` **not** ( `$` | `@` | `{` ) | *regular_expression_unescaped_sequence* | *regular_expression_escape_sequence* | *line_terminator_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn regular_expression_character(i: Input) -> SegmentResult {
    alt((
        map(none_of("/#\\"), |c| Segment::Char(c)),
        map(regular_expression_unescaped_sequence, |s| {
            Segment::String(s.to_string())
        }),
        map(regular_expression_escape_sequence, |_| Segment::Char('/')),
        map(line_terminator_escape_sequence, |_| {
            Segment::String("".to_owned())
        }),
        map(interpolated_character_sequence, |e| Segment::expr(e)),
        map(char('#'), |c| Segment::Char(c)),
    ))(i)
}

/// `\` *regular_expression_unescaped_character*
pub(crate) fn regular_expression_unescaped_sequence(i: Input) -> LexResult {
    recognize(tuple((char('\\'), regular_expression_unescaped_character)))(i)
}

/// *source_character* **but not** ( 0x0d | 0x0a ) | 0x0d **not** 0x0a
pub(crate) fn regular_expression_unescaped_character(i: Input) -> CharResult {
    alt((
        none_of("\x0d\x0a"),
        terminated(char('\x0d'), not(peek(char('\x0a')))),
    ))(i)
}

/// `\` `/`
pub(crate) fn regular_expression_escape_sequence(i: Input) -> LexResult {
    tag("\\/")(i)
}

/// `i` | `m`
pub(crate) fn regular_expression_option(i: Input) -> CharResult {
    one_of("im")(i)
}
