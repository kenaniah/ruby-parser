use crate::lexer::*;
use crate::parsers::program::{line_terminator, whitespace};
use crate::parsers::token::literal::string::double::double_escape_sequence;
use crate::parsers::token::literal::string::double::interpolated_character_sequence;
use crate::parsers::token::literal::string::quoted::literal_beginning_delimiter;
use crate::parsers::token::literal::string::quoted::literal_ending_delimiter;
use crate::parsers::token::literal::string::quoted::non_escaped_literal_character;
use crate::parsers::token::literal::string::quoted::wrap_quote_delimiter;

/// *quoted_non_expanded_array_constructor* | *quoted_expanded_array_constructor*
pub(crate) fn array_literal(i: Input) -> NodeResult {
    stub(i)
}

/// `%w` *literal_beginning_delimiter* *non_expanded_array_content* *literal_ending_delimiter*
pub(crate) fn quoted_non_expanded_array_constructor(i: Input) -> LexResult {
    stub_p(i)
}

/// *quoted_array_item_separator_list*? *non_expanded_array_item_list*? *quoted_array_item_separator_list*?
pub(crate) fn non_expanded_array_content(i: Input) -> LexResult {
    stub_p(i)
}

/// *non_expanded_array_item* ( *quoted_array_item_separator_list* *non_expanded_array_item* )*
pub(crate) fn non_expanded_array_item_list(i: Input) -> LexResult {
    stub_p(i)
}

/// *quoted_array_item_separator*+
pub(crate) fn quoted_array_item_separator_list(i: Input) -> LexResult {
    recognize(many1(quoted_array_item_separator))(i)
}

/// *whitespace* | *line_terminator*
pub(crate) fn quoted_array_item_separator(i: Input) -> LexResult {
    alt((whitespace, line_terminator))(i)
}

/// *non_expanded_array_item_character*+
pub(crate) fn non_expanded_array_item(i: Input) -> LexResult {
    stub_p(i)
}

/// *non_escaped_array_character* | *non_expanded_array_escape_sequence*
pub(crate) fn non_expanded_array_item_character(i: Input) -> LexResult {
    stub_p(i)
}

/// *non_escaped_literal_character* **but not** *quoted_array_item_separator*
pub(crate) fn non_escaped_array_character(i: Input) -> CharResult {
    let (i, _) = peek(not(quoted_array_item_separator))(i)?;
    non_escaped_literal_character(i)
}

/// *non_expanded_literal_escape_sequence* | `\` *quoted_array_item_separator*
pub(crate) fn non_expanded_array_escape_sequence(i: Input) -> LexResult {
    stub_p(i)
}

/// `%W` *literal_beginning_delimiter* *expanded_array_content* *literal_ending_delimiter*
pub(crate) fn quoted_expanded_array_constructor(i: Input) -> Parsed<Vec<Interpolatable>> {
    preceded(
        tag("%W"),
        wrap_quote_delimiter(delimited(
            literal_beginning_delimiter,
            expanded_array_content,
            literal_ending_delimiter,
        )),
    )(i)
}

/// *quoted_array_item_separator_list*? *expanded_array_item_list*? *quoted_array_item_separator_list*?
pub(crate) fn expanded_array_content(i: Input) -> Parsed<Vec<Interpolatable>> {
    map(
        delimited(
            opt(quoted_array_item_separator_list),
            opt(expanded_array_item_list),
            opt(quoted_array_item_separator_list),
        ),
        |content| content.unwrap_or(vec![]),
    )(i)
}

/// *expanded_array_item* ( *quoted_array_item_separator_list* *expanded_array_item* )*
pub(crate) fn expanded_array_item_list(i: Input) -> Parsed<Vec<Interpolatable>> {
    map(
        tuple((
            expanded_array_item,
            many0(preceded(
                quoted_array_item_separator_list,
                expanded_array_item,
            )),
        )),
        |(first, mut vec)| {
            vec.insert(0, first);
            vec
        },
    )(i)
}

/// *expanded_array_item_character*+
pub(crate) fn expanded_array_item(i: Input) -> InterpolatableResult {
    map(many1(expanded_array_item_character), |contents| {
        Interpolatable::from(contents)
    })(i)
}

/// *non_escaped_array_item_character* | `#` **not** ( `$` | `@` | `{` ) | *expanded_array_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn expanded_array_item_character(i: Input) -> SegmentResult {
    alt((
        map(non_escaped_array_character, |c| Segment::Char(c)),
        map(expanded_array_escape_sequence, |s| Segment::String(s)),
        map(interpolated_character_sequence, |e| Segment::expr(e)),
        map(char('#'), |c| Segment::Char(c)),
    ))(i)
}

/// *source_character* **but not** ( *quoted_array_item_separator* | `\` | `#` )
pub(crate) fn non_escaped_array_item_character(i: Input) -> CharResult {
    let (i, _) = peek(not(quoted_array_item_separator))(i)?;
    none_of("\\#")(i)
}

/// *double_escape_sequence* | `\` *quoted_array_item_separator*
pub(crate) fn expanded_array_escape_sequence(i: Input) -> StringResult {
    alt((
        double_escape_sequence,
        map(preceded(char('\\'), quoted_array_item_separator), |s| {
            s.into()
        }),
    ))(i)
}

fn stub_p(i: Input) -> LexResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
