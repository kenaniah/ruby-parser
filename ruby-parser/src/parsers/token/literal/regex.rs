use crate::lexer::*;

/// `/` *regular_expression_body* `/` *regular_expression_option** | `%r` *literal_beginning_delimiter* *expanded_literal_string** *literal_ending_delimiter* *regular_expression_option**
pub(crate) fn regular_expression_literal(i: Input) -> NodeResult {
    stub(i)
}

/// *regular_expression_character**
pub(crate) fn regular_expression_body(i: Input) -> LexResult {
    stub_p(i)
}

/// *source_character* **but not** ( `/` | `#` | `\` ) | `#` **not** ( `$` | `@` | `{` ) | *regular_expression_unescaped_sequence* | *regular_expression_escape_sequence* | *line_terminator_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn regular_expression_character(i: Input) -> LexResult {
    stub_p(i)
}

/// `\` *regular_expression_unescaped_character*
pub(crate) fn regular_expression_unescaped_sequence(i: Input) -> LexResult {
    stub_p(i)
}

/// *source_character* **but not** ( 0x0d | 0x0a ) | 0x0d **not** 0x0a
pub(crate) fn regular_expression_unescaped_character(i: Input) -> LexResult {
    stub_p(i)
}

/// `\` `/`
pub(crate) fn regular_expression_escape_sequence(i: Input) -> LexResult {
    stub_p(i)
}

/// `i` | `m`
pub(crate) fn regular_expression_option(i: Input) -> LexResult {
    stub_p(i)
}

fn stub_p(i: Input) -> LexResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}