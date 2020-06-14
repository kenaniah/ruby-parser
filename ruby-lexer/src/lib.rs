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
extern crate nom_locate;

#[macro_use]
mod macros;
pub mod lexers;
mod input;

/// Internal enum used by the numeric_literal parser
#[derive(Debug, PartialEq)]
pub(crate) enum Numeric {
    Integer(isize),
    Float(f64),
}

/// Defines the tokens that are returned as a result of lexing
#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(isize),
    Float(f64),
}

/// The type used to describe the lexer's input
//pub type Input<'a> = input::TrackedLocation<&'a str>;
pub type Input<'a> = &'a str;

/// Describes a parsed token
pub type TokenResult<'a> = nom::IResult<Input<'a>, Token>;

/// Describes a parsed numeric literal
pub(crate) type NumericResult<'a> = nom::IResult<Input<'a>, Numeric>;

/// Describes a parsed character
pub(crate) type CharResult<'a> = nom::IResult<Input<'a>, char>;

/// Describes a parsed string
pub(crate) type StringResult<'a> = nom::IResult<Input<'a>, String>;
