use crate::{Numeric, Token, TrackedLocation};
use nom::IResult;

/// Intermediate type used to build the result types for lexing combinators
type Parsed<'a, T> = IResult<Input<'a>, T>;

/// Describes a generic result type on which others could be built
type VecResult<'a, T> = Parsed<'a, Vec<T>>;

/// Describes the lexer's input type
pub type Input<'a> = TrackedLocation<&'a str>;

/// Describes a single parsed token
pub type TokenResult<'a> = Parsed<'a, Token>;

/// Describes a list of parsed tokens
pub type TokenVecResult<'a> = VecResult<'a, Token>;

/// Describes a parsed character
pub(crate) type CharResult<'a> = Parsed<'a, char>;

/// Describes a parsed numeric literal
pub(crate) type NumericResult<'a> = Parsed<'a, Numeric>;

/// Describes a nom-compatible parsing result type (input and output types match)
pub(crate) type ParseResult<'a> = Parsed<'a, Input<'a>>;

/// Describes a parsed statement
pub(crate) type StatementResult<'a> = Parsed<'a, Vec<Token>>;

/// Describes a parsed string
pub(crate) type StringResult<'a> = Parsed<'a, String>;
