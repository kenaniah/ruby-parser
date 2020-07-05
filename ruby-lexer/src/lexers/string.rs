/*!
Provides support for lexing Ruby's string literal formats.
!*/
use crate::{Input, Interpolatable, Token, TokenResult};
use nom::branch::alt;
use nom::combinator::map;

pub(crate) mod double;
pub(crate) mod single;

/// *single_quoted_string* | *double_quoted_string* | *quoted_non_expanded_literal_string* | *quoted_expanded_literal_string* | *here_document* | *external_command_execution*
pub fn string_literal(i: Input) -> TokenResult {
    alt((
        map(single::single_quoted_string, |s| {
            Token::SingleQuotedString(s)
        }),
        map(double::double_quoted_string, |s| {
            match s {
                Interpolatable::String(s) => Token::DoubleQuotedString(s),
                Interpolatable::Interpolated(i) => Token::InterpolatedString(i)
            }
        })
        // quoted_non_expanded_literal_string,
        // quoted_expanded_literal_string,
        // here_document,
        // external_command_execution,
    ))(i)
}
