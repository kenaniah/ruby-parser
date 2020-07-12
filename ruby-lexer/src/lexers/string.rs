/*!
Provides support for lexing Ruby's string literal formats.
!*/
use crate::*;
use nom::branch::alt;
use nom::combinator::map;

pub(crate) mod command;
pub(crate) mod double;
pub(crate) mod quoted;
pub(crate) mod single;

/// *single_quoted_string* | *double_quoted_string* | *quoted_non_expanded_literal_string* | *quoted_expanded_literal_string* | *here_document* | *external_command_execution*
pub fn string_literal(i: Input) -> TokenResult {
    alt((
        map(single::single_quoted_string, |s| Token::String(s)),
        map(double::double_quoted_string, |s| match s {
            Interpolatable::String(s) => Token::String(s),
            Interpolatable::Interpolated(i) => Token::InterpolatedString(i),
        }),
        map(quoted::quoted_non_expanded_literal_string, |s| {
            Token::String(s)
        }),
        map(quoted::quoted_expanded_literal_string, |s| match s {
            Interpolatable::String(s) => Token::String(s),
            Interpolatable::Interpolated(i) => Token::InterpolatedString(i),
        }),
        // here_document,
        map(command::external_command_execution, |s| match s {
            Interpolatable::String(s) => Token::ExternalCommand(s),
            Interpolatable::Interpolated(i) => Token::InterpolatedExternalCommand(i),
        }),
    ))(i)
}
