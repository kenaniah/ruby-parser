use crate::lexers::string::double::double_quoted_string;
use crate::lexers::string::single::single_quoted_string;
use crate::{CharResult, Input, Token, TokenResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, one_of};
use nom::combinator::{map, recognize, verify};
use nom::multi::many0;
use nom::sequence::tuple;

/// *symbol_literal* | *dynamic_symbol*
pub fn symbol(i: Input) -> TokenResult {
    alt((symbol_literal, dynamic_symbol))(i)
}

/// `:` *symbol_name*
pub(crate) fn symbol_literal(i: Input) -> TokenResult {
    map(recognize(tuple((char(':'), symbol_name))), |s| {
        Token::Symbol((*s).to_owned())
    })(i)
}

/// `:` *single_quoted_string*  | `:` *double_quoted_string* | `%s` *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn dynamic_symbol(i: Input) -> TokenResult {
    alt((
        map(recognize(tuple((char(':'), single_quoted_string))), |s| {
            Token::Symbol((*s).to_owned())
        }),
        stub_token
        // map(tuple((char(':'), double_quoted_string)), |t| {
        //     let mut vec:  = t.1;
        //     vec.
        //     Token::InterpolatedSymbol(vec)
        // }),
    ))(i)
}

/// *identifier* | *operator_method_name* | *keyword*
pub(crate) fn symbol_name(i: Input) -> TokenResult {
    stub_token(i)
}

fn stub_token(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Complete)))
}
