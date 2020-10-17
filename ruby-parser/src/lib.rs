/*!
# Ruby Parser - a library for parsing Ruby syntax

ruby-parser is a library that provides the APIs needed to lex the Ruby programming language's
syntax into a stream of tokens.

## Parser combinators

This library is provided as a set of parser combinator functions, powered by [nom](https://docs.rs/nom/).
All of the parser combinators are structured to semantically reflect the ISO Ruby specification,
but minor deviations from the spec have been made when necessary (namely, re-ordering alternative
parsers to consume the largest production first).

The top-level parser combinators that return tokens are publically exported within the parsers module.
!*/

extern crate nom;

#[macro_use]
mod macros;
pub mod ast;
pub mod lexer;
mod parsers;

pub use nom::error::ErrorKind;

/// Parses a ruby program
pub fn parse(i: lexer::Input) -> lexer::ProgramResult {
    parsers::program::program(i)
}
