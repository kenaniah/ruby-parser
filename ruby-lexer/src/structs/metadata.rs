/// Tracks lexer-specific metadata
#[derive(Debug, Clone, Copy, Default)]
pub struct Metadata<'a> {
    pub file: Option<&'a str>,
    pub quote_delimiter: Option<char>,
}
