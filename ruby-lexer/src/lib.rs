extern crate nom;

#[macro_use]
mod macros;
pub mod parsers;

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
    Float(f64)
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
