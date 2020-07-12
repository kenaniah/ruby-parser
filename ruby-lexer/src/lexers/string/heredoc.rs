use crate::*;

/// *heredoc_start_line* *heredoc_body* *heredoc_end_line*
pub(crate) fn here_document(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *heredoc_signifier* *rest_of_line*
pub(crate) fn heredoc_start_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// `<<` *heredoc_delimiter_specifier*
pub(crate) fn heredoc_signifier(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *line_content*? *line_terminator*
pub(crate) fn rest_of_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *heredoc_body_line**
pub(crate) fn heredoc_body(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *comment_line* **but not** *heredoc_end_line*
pub(crate) fn heredoc_body_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// ( `-` | `~` )? *heredoc_delimiter*
pub(crate) fn heredoc_delimiter_specifier(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *non_quoted_delimiter* | *single_quoted_delimiter* | *double_quoted_delimiter* | *command_quoted_delimiter*
pub(crate) fn heredoc_delimiter(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *non_quoted_delimiter_identifier*
pub(crate) fn non_quoted_delimiter(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *identifier_character**
pub(crate) fn non_quoted_delimiter_identifier(i: Input) -> InterpolatableResult {
    stub(i)
}

/// `'` *single_quoted_delimiter_identifier* `'`
pub(crate) fn single_quoted_delimiter(i: Input) -> InterpolatableResult {
    stub(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( `'` | *line_terminator* ) )*
pub(crate) fn single_quoted_delimiter_identifier(i: Input) -> InterpolatableResult {
    stub(i)
}

/// `"` *double_quoted_delimiter_identifier* `"`
pub(crate) fn double_quoted_delimiter(i: Input) -> InterpolatableResult {
    stub(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( `"` | *line_terminator* ) )*
pub(crate) fn double_quoted_delimiter_identifier(i: Input) -> InterpolatableResult {
    stub(i)
}

/// ``` *command_quoted_delimiter_identifier* ```
pub(crate) fn command_quoted_delimiter(i: Input) -> InterpolatableResult {
    stub(i)
}

/// ( ( *source_character* *source_character*? ) **but not** ( ``` | *line_terminator* ) )*
pub(crate) fn command_quoted_delimiter_identifier(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *indented_heredoc_end_line* | *non_indented_heredoc_end_line*
pub(crate) fn heredoc_end_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// [beginning of a line] *whitespace** *heredoc_delimiter_identifier* *line_terminator*
pub(crate) fn indented_heredoc_end_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// [beginning of a line] *heredoc_delimiter_identifier* *line_terminator*
pub(crate) fn non_indented_heredoc_end_line(i: Input) -> InterpolatableResult {
    stub(i)
}

/// *non_quoted_delimiter_identifier* | *single_quoted_delimiter_identifier* | *double_quoted_delimiter_identifier* | *command_quoted_delimiter_identifier*
pub(crate) fn heredoc_delimiter_identifier(i: Input) -> InterpolatableResult {
    stub(i)
}

fn stub(i: Input) -> InterpolatableResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
