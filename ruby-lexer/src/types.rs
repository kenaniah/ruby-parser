use crate::{Interpolated, Numeric, Token, TrackedLocation};
use nom::IResult;

/// Intermediate type used to build the result types for lexing combinators
type Parsed<'a, T> = IResult<Input<'a>, T>;

/// Describes a list of tokens that make up an expression
pub type Expression = Vec<Token>;

/// Describes the lexer's input type
pub type Input<'a> = TrackedLocation<&'a str>;

/// Describes a list of statements
pub type StatementList = Vec<Token>;

/// Describes a parsed character
pub(crate) type CharResult<'a> = Parsed<'a, char>;

/// Describes a potentially interpolated character
pub(crate) type InterpolatedResult<'a> = Parsed<'a, Interpolated>;

/// Describes a parsed numeric literal
pub(crate) type NumericResult<'a> = Parsed<'a, Numeric>;

/// Describes a nom-compatible parsing result type (input and output types match)
pub(crate) type ParseResult<'a> = Parsed<'a, Input<'a>>;

/// Describes a parsed string
pub(crate) type StringResult<'a> = Parsed<'a, String>;

/// Describes a single parsed token
pub(crate) type TokenResult<'a> = Parsed<'a, Token>;
