/*!
# Ruby Lexer - a library for parsing Ruby syntax

ruby-lexer is a library that provides the APIs needed to lex the Ruby programming language's
syntax into a stream of tokens.

## Parser combinators

This library is provided as a set of parser combinator functions, powered by [nom](https://docs.rs/nom/).
All of the parser combinators are structured to semantically reflect the ISO Ruby specification,
but minor deviations from the spec have been made when necessary (namely, re-ordering alternative
parsers to consume the largest production first).

The top-level parser combinators that return tokens are publically exported within the lexers module.
!*/

extern crate nom;

#[macro_use]
mod macros;
mod enums;
pub mod lexers;
mod structs;
mod types;

pub use enums::{Interpolatable, Token};
pub(crate) use enums::{Numeric, Segment};
pub use nom::error::ErrorKind;
pub use structs::tracked_location::TrackedLocation;
pub use structs::metadata::Metadata;
pub(crate) use types::{
    CharResult, InterpolatableResult, NumericResult, ParseResult, SegmentResult, StringResult,
    TokenResult,
};
pub use types::{Expression, Input, StatementList};
