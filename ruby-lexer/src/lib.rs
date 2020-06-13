/*!
# Ruby Lexer - a library for parsing Ruby syntax

ruby-lexer is a library that provides the APIs needed to lex the Ruby programming language's
syntax into a stream of tokens.

## Parser combinators

This library is provided as a set of parser combinator functions, powered by [nom](https://docs.rs/nom/).
All of the parser combinators are structured to semantically reflect the ISO Ruby specification,
but minor deviations from the spec have been made when necessary (namely, re-ordering alternative
parsers to consume the largest production first).

## Prelude

As a parser combinator library, essentially everything has been marked as public. The most useful
parts of the library have been re-exported under the prelude module.
!*/

extern crate nom;
extern crate nom_locate;

#[macro_use]
mod macros;
pub mod lexers;
pub mod prelude;

/// Internal enum used by the numeric_literal parser
#[derive(Debug, PartialEq)]
pub enum Numeric {
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
pub type Input<'a> = &'a str;

/// Describes a parsed numeric literal
pub type NumericResult<'a> = nom::IResult<Input<'a>, Numeric>;

/// Describes a parsed character
pub type CharResult<'a> = nom::IResult<Input<'a>, char>;

/// Describes a parsed string
pub type StringResult<'a> = nom::IResult<Input<'a>, String>;

/// Describes a parsed token
pub type TokenResult<'a> = nom::IResult<Input<'a>, Token>;
