//! Provides types / enums / structs for defining parser combinators

mod heredoc;
mod interpolable;
mod metadata;
mod segment;
mod tracked_location;

pub use crate::ast::{Identifier, IdentifierKind, Interpolated, Literal, Node};
pub use heredoc::{HeredocIndentation, HeredocMetadata, HeredocQuoteType};
pub use interpolable::Interpolatable;
pub use metadata::Metadata;
pub use segment::Segment;
pub use tracked_location::TrackedLocation;

/// Describes the parser's input type
pub type Input<'a> = TrackedLocation<&'a str, Metadata<'a>>;

/// Intermediate type used to build the result types for lexing combinators
pub(crate) type Parsed<'a, T> = nom::IResult<Input<'a>, T>;

// Describes a parsed AST node
pub(crate) type NodeResult<'a> = Parsed<'a, Node>;

// Describes a vector of parsed nodes
pub(crate) type NodeListResult<'a> = Parsed<'a, Vec<Node>>;

/// Describes a parsed literal
pub(crate) type LiteralResult<'a> = Parsed<'a, Literal>;

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

pub(crate) fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

pub(crate) fn stub_list(i: Input) -> NodeListResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
