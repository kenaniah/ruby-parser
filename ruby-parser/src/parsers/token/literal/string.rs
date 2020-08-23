/*!
Provides support for lexing Ruby's string literal formats.
!*/
use crate::*;
use nom::branch::alt;
use nom::combinator::map;

pub(crate) mod character;
pub(crate) mod command;
pub(crate) mod double;
pub(crate) mod heredoc;
pub(crate) mod quoted;
pub(crate) mod single;

pub(crate) use character::character_literal;
pub(crate) use command::external_command_execution;
pub(crate) use double::double_quoted_string;
pub(crate) use heredoc::here_document;
pub(crate) use quoted::quoted_expanded_literal_string;
pub(crate) use quoted::quoted_non_expanded_literal_string;
pub(crate) use single::single_quoted_string;

/// *single_quoted_string* | *double_quoted_string* | *quoted_non_expanded_literal_string* | *quoted_expanded_literal_string* | *here_document* | *external_command_execution*
pub(crate) fn string_literal(i: Input) -> TokenResult {
    alt((
        map(single_quoted_string, |s| Token::Literal(Literal::String(s))),
        map(double_quoted_string, |s| match s {
            Interpolatable::String(s) => Token::Literal(Literal::String(s)),
            Interpolatable::Interpolated(i) => Token::InterpolatedString(i),
        }),
        map(quoted_non_expanded_literal_string, |s| Token::Literal(Literal::String(s))),
        map(quoted_expanded_literal_string, |s| match s {
            Interpolatable::String(s) => Token::Literal(Literal::String(s)),
            Interpolatable::Interpolated(i) => Token::InterpolatedString(i),
        }),
        here_document,
        map(external_command_execution, |s| match s {
            Interpolatable::String(s) => Token::Literal(Literal::ExternalCommand(s)),
            Interpolatable::Interpolated(i) => Token::InterpolatedExternalCommand(i),
        }),
        map(character_literal, |s| Token::Literal(Literal::String(s))),
    ))(i)
}
