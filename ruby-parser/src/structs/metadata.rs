use crate::{HeredocIndentation, HeredocQuoteType};

/// Tracks lexer-specific metadata
#[derive(Debug, Clone, Default)]
pub struct Metadata<'a> {
    /// Provides a reference to the name of the file being parsed
    pub file: Option<&'a str>,
    /// Tracks the delimiter used when parsing a quoted string
    pub(crate) quote_delimiter: Option<char>,
    /// Tracks heredoc-specific lexer state
    pub(crate) heredoc: Option<Box<HeredocMetadata<'a>>>,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metadata_size() {
        assert_eq!(32, std::mem::size_of::<Metadata>());
    }
}
