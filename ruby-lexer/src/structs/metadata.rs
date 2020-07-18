use crate::{HeredocDelimiter, HeredocIndent};

/// Tracks lexer-specific metadata
#[derive(Debug, Clone, Copy, Default)]
pub struct Metadata<'a> {
    /// Provides a reference to the name of the file being parsed
    pub file: Option<&'a str>,
    /// Tracks the delimiter used when parsing a quoted string
    pub(crate) quote_delimiter: Option<char>,
    /// Tracks the type of delimiter used when parsing a heredoc
    pub(crate) heredoc_delimiter: Option<HeredocDelimiter>,
    /// Tracks the identiation mode used when parsing a heredoc
    pub(crate) heredoc_indentation: Option<HeredocIndent>,
    /// Tracks a reference to the heredoc identifier when parsing a heredoc
    pub(crate) heredoc_identifier: Option<&'a str>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metadata_size() {
        assert_eq!(40, std::mem::size_of::<Metadata>());
    }
}
