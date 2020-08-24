use super::heredoc::HeredocMetadata;

/// Tracks parser-specific metadata
#[derive(Debug, Clone, Default)]
pub struct Metadata<'a> {
    /// Provides a reference to the name of the file being parsed
    pub file: Option<&'a str>,
    /// Tracks the delimiter used when parsing a quoted string
    pub(crate) quote_delimiter: Option<char>,
    /// Tracks heredoc-specific lexer state
    pub(crate) heredoc: Option<Box<HeredocMetadata<'a>>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_metadata_size() {
        assert_eq!(32, std::mem::size_of::<Metadata>());
    }
}
