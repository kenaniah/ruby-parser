//! Provides types / enums / structs for defining parser combinators

mod heredoc;
mod interpolable;
mod metadata;
mod nom_prelude;
mod segment;
mod tracked_location;

pub use crate::ast::{
    Identifier, IdentifierKind, Interpolated, Literal, Node, Parameter, Program, WhenClause,
};
pub use heredoc::{HeredocIndentation, HeredocMetadata, HeredocQuoteType};
pub use interpolable::Interpolatable;
pub use metadata::Metadata;
pub(crate) use nom_prelude::*;
pub use segment::Segment;
pub use tracked_location::TrackedLocation;

/// Describes the parser's input type
pub type Input<'a> = TrackedLocation<&'a str, Metadata<'a>>;

/// Intermediate type used to build the result types for lexing combinators
pub(crate) type Parsed<'a, T> = nom::IResult<Input<'a>, T>;

// Describes a parsed program
pub(crate) type ProgramResult<'a> = Parsed<'a, Program>;

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

/// Describes a parsed string
pub(crate) type StringResult<'a> = Parsed<'a, String>;

/// Describes a parsed identifier
pub(crate) type IdentifierResult<'a> = Parsed<'a, Identifier>;
