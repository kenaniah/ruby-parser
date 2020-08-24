/// Represents the delimiter type of a heredoc
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HeredocQuoteType {
    Unquoted,
    SingleQuoted,
    DoubleQuoted,
    CommandQuoted,
}

/// Represents the indentation mode of a heredoc
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum HeredocIndentation {
    Unindented,
    Indented,
    FullyIndented,
}

/// Tracks heredoc-specific metadata when parsing a heredoc
#[derive(Debug, Clone, Default)]
pub struct HeredocMetadata<'a> {
    /// Tracks the type of quotation used when parsing a heredoc
    pub(crate) quote_type: Option<HeredocQuoteType>,
    /// Tracks the identiation mode used when parsing a heredoc
    pub(crate) indentation: Option<HeredocIndentation>,
    /// Tracks a reference to the heredoc's identifier when parsing a heredoc
    pub(crate) identifier: Option<&'a str>,
    /// Allows heredoc metadata to leak for test verification purposes
    #[cfg(test)]
    pub(crate) should_leak: bool,
}
