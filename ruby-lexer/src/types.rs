use crate::{Numeric, Token, TrackedLocation};

/// Describes the lexer's input type
pub type Input<'a> = TrackedLocation<&'a str>;

/// Describes a list of parsed tokens
pub type TokenStreamResult<'a> = nom::IResult<Input<'a>, Vec<Token>>;

/// Describes a single parsed token
pub type TokenResult<'a> = nom::IResult<Input<'a>, Token>;

/// Describes a nom-compatible parsing result type (input and output types match)
pub(crate) type ParseResult<'a> = nom::IResult<Input<'a>, Input<'a>>;

/// Describes a parsed numeric literal
pub(crate) type NumericResult<'a> = nom::IResult<Input<'a>, Numeric>;

/// Describes a parsed character
pub(crate) type CharResult<'a> = nom::IResult<Input<'a>, char>;

/// Describes a parsed string
pub(crate) type StringResult<'a> = nom::IResult<Input<'a>, String>;
