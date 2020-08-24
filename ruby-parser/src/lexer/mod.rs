use crate::{Interpolatable, Segment, Token};

pub(crate) mod metadata;
mod tracked_location;

pub use metadata::Metadata;
pub use tracked_location::TrackedLocation;

/// Describes the parser's input type
pub type Input<'a> = TrackedLocation<&'a str, Metadata<'a>>;

/// Intermediate type used to build the result types for lexing combinators
pub(crate) type Parsed<'a, T> = nom::IResult<Input<'a>, T>;

/// Describes a list of tokens that make up an expression
pub type Expression = Vec<Token>;

/// Describes a list of statements
pub type StatementList = Vec<Token>;

/// Describes a parsed character
pub(crate) type CharResult<'a> = Parsed<'a, char>;

/// Describes an interpolated result
pub(crate) type InterpolatableResult<'a> = Parsed<'a, Interpolatable>;

/// Describes a nom-compatible parsing result type (input and output types match)
pub(crate) type LexResult<'a> = Parsed<'a, Input<'a>>;

/// Describes a segment of something that may be interpolated
pub(crate) type SegmentResult<'a> = Parsed<'a, Segment>;

/// Describes a segment of something that may be interpolated
pub(crate) type SegmentVecResult<'a> = Parsed<'a, Vec<Segment>>;

/// Describes a parsed string
pub(crate) type StringResult<'a> = Parsed<'a, String>;

/// Describes a single parsed token
pub(crate) type TokenResult<'a> = Parsed<'a, Token>;

/// Describes a list of parsed tokens
pub(crate) type TokenizedResult<'a> = Parsed<'a, Vec<Token>>;
