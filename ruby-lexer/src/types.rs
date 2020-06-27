use crate::{Numeric, Token, TrackedLocation};

/// The type used to describe the lexer's input
pub type Input<'a> = TrackedLocation<&'a str>;

/// Describes a parsed token
pub type TokenResult<'a> = nom::IResult<Input<'a>, Token>;

/// Describes a parse result
pub(crate) type ParseResult<'a> = nom::IResult<Input<'a>, Input<'a>>;

/// Describes a parsed numeric literal
pub(crate) type NumericResult<'a> = nom::IResult<Input<'a>, Numeric>;

/// Describes a parsed character
pub(crate) type CharResult<'a> = nom::IResult<Input<'a>, char>;

/// Describes a parsed string
pub(crate) type StringResult<'a> = nom::IResult<Input<'a>, String>;
