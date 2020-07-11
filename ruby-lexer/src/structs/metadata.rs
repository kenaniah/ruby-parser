/// Tracks location information and user-defined metadata for nom's source input.
#[derive(Debug, Clone, Copy, Default)]
pub struct Metadata<'a> {
    file: Option<&'a str>,
    quote_delimiter: Option<char>,
}
