/*!
Provides support for lexing Ruby's string literal formats.
!*/

use crate::{Input, TokenResult};
use nom::branch::alt;

pub(crate) mod double;
pub(crate) mod single;

/// *single_quoted_string* | *double_quoted_string* | *quoted_non_expanded_literal_string* | *quoted_expanded_literal_string* | *here_document* | *external_command_execution*
pub fn string_literal(i: Input) -> TokenResult {
    alt((
        single::single_quoted_string,
        double::double_quoted_string,
        // quoted_non_expanded_literal_string,
        // quoted_expanded_literal_string,
        // here_document,
        // external_command_execution,
    ))(i)
}
