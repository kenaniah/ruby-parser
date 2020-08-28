use crate::lexer::*;
use nom::branch::alt;

pub(crate) mod array;
pub(crate) mod numeric;
pub(crate) mod regex;
pub(crate) mod string;
pub(crate) mod symbol;

pub(crate) use array::array_literal;
pub(crate) use numeric::numeric_literal;
pub(crate) use regex::regular_expression_literal;
pub(crate) use string::string_literal;
pub(crate) use symbol::symbol;

/// *numeric_literal* | *string_literal* | *array_literal* | *regular_expression_literal* | *symbol*
pub(crate) fn literal(i: Input) -> NodeResult {
    alt((
        numeric_literal,
        string_literal,
        array_literal,
        regular_expression_literal,
        symbol,
    ))(i)
}
