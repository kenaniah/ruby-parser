use crate::ast::NodeResult;
use crate::*;

/// *relational_expression* | *relational_expression* [ no line terminator here ] ( `<=>` | `===` | `==` | `!=` | `=~` | `!~` ) *relational_expression*
pub(crate) fn equality_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *bitwise_or_expression* | *relational_expression* [ no line terminator here ] ( `>=` | `>` | `<=` | `<` ) *bitwise_or_expression*
pub(crate) fn relational_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *bitwise_and_expression* | *bitwise_or_expression* [ no line terminator here ] ( `|` | `^` ) *bitwise_and_expression*
pub(crate) fn bitwise_or_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *bitwise_shift_expression* | *bitwise_and_expression* [ no line terminator here ] `&` *bitwise_shift_expression*
pub(crate) fn bitwise_and_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *additive_expression* | *bitwise_shift_expression* [ no line terminator here ] ( `<<` | `>>` ) *additive_expression*
pub(crate) fn bitwise_shift_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *multiplicative_expression* | *additive_expression* [ no line terminator here ] ( `+` | `-` ) *multiplicative_expression*
pub(crate) fn additive_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *unary_minus_expression* | *multiplicative_expression* [ no line terminator here ] ( `*` | `/` | `%` ) *unary_minus_expression*
pub(crate) fn multiplicative_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *unary_expression* | *unary_expression* [ no line terminator here ] `**` *power_expression*
pub(crate) fn power_expression(i: Input) -> NodeResult {
    stub(i)
}

fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
