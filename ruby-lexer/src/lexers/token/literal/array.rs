use crate::*;

/// *quoted_non_expanded_array_constructor* | *quoted_expanded_array_constructor*
pub fn array_literal(i: Input) -> TokenResult {
    stub(i)
}

/// `%w` *literal_beginning_delimiter* *non_expanded_array_content* *literal_ending_delimiter*
pub(crate) fn quoted_non_expanded_array_constructor(i: Input) -> ParseResult {
    stub_p(i)
}

/// *quoted_array_item_separator_list*? *non_expanded_array_item_list*? *quoted_array_item_separator_list*?
pub(crate) fn non_expanded_array_content(i: Input) -> ParseResult {
    stub_p(i)
}

/// *non_expanded_array_item* ( *quoted_array_item_separator_list* *non_expanded_array_item* )*
pub(crate) fn non_expanded_array_item_list(i: Input) -> ParseResult {
    stub_p(i)
}

/// *quoted_array_item_separator*+
pub(crate) fn quoted_array_item_separator_list(i: Input) -> ParseResult {
    stub_p(i)
}

/// *whitespace* | *line_terminator*
pub(crate) fn quoted_array_item_separator(i: Input) -> ParseResult {
    stub_p(i)
}

/// *non_expanded_array_item_character*+
pub(crate) fn non_expanded_array_item(i: Input) -> ParseResult {
    stub_p(i)
}

/// *non_escaped_array_character* | *non_expanded_array_escape_sequence*
pub(crate) fn non_expanded_array_item_character(i: Input) -> ParseResult {
    stub_p(i)
}

/// *non_escaped_literal_character* **but not** *quoted_array_item_separator*
pub(crate) fn non_escaped_array_character(i: Input) -> ParseResult {
    stub_p(i)
}

/// *non_expanded_literal_escape_sequence* | `\` *quoted_array_item_separator*
pub(crate) fn non_expanded_array_escape_sequence(i: Input) -> ParseResult {
    stub_p(i)
}

/// `%W` *literal_beginning_delimiter* *expanded_array_content* *literal_ending_delimiter*
pub(crate) fn quoted_expanded_array_constructor(i: Input) -> ParseResult {
    stub_p(i)
}

/// *quoted_array_item_separator_list*? *expanded_array_item_list*? *quoted_array_item_separator_list*?
pub(crate) fn expanded_array_content(i: Input) -> ParseResult {
    stub_p(i)
}

/// *expanded_array_item* ( *quoated_array_item_separator_list* *expanded_array_item* )*
pub(crate) fn expanded_array_item_list(i: Input) -> ParseResult {
    stub_p(i)
}

/// *expanded_array_item_character*+
pub(crate) fn expanded_array_item(i: Input) -> ParseResult {
    stub_p(i)
}

/// *non_escaped_array_item_character* | `#` **not** ( `$` | `@` | `{` ) | *expanded_array_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn expanded_array_item_character(i: Input) -> ParseResult {
    stub_p(i)
}

/// *source_character* **but not** ( *quoted_array_item_separator* | `\` | `#` )
pub(crate) fn non_escaped_array_item_character(i: Input) -> ParseResult {
    stub_p(i)
}

/// *double_escape_sequence* | `\` *quoted_array_item_separator*
pub(crate) fn expanded_array_escape_sequence(i: Input) -> ParseResult {
    stub_p(i)
}

fn stub_p(i: Input) -> ParseResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
