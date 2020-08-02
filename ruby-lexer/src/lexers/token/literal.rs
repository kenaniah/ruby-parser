use crate::*;
use nom::branch::alt;

mod array;
mod numeric;
mod regex;
mod string;
mod symbol;

pub use array::array_literal;
pub use numeric::numeric_literal;
pub use regex::regular_expression_literal;
pub use string::string_literal;
pub use symbol::symbol;

/// *numeric_literal* | *string_literal* | *array_literal* | *regular_expression_literal* | *symbol*
pub fn literal(i: Input) -> TokenResult {
    alt((
        numeric_literal,
        string_literal,
        array_literal,
        regular_expression_literal,
        symbol,
    ))(i)
}
