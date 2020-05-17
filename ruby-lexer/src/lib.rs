extern crate nom;

#[macro_use]
mod macros;
pub mod parsers;
pub mod token;

#[derive(Debug, PartialEq)]
pub enum Numeric {
    Integer(isize),
    Float(f64),
}

pub use token::Token;

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
