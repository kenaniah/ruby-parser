use nom;

mod parsers;
mod token;

pub type ParseResult<'a> = nom::IResult<&'a str, &'a str>;
