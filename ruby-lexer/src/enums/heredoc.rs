/// Represents the delimiter type of a heredoc
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum HeredocDelimiter {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
    CommandQuoted,
}

/// Represents the indentation mode of a heredoc
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum HeredocIndent {
    Unindented,
    Indented,
    FullyIntented,
}
