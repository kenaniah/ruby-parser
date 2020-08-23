use crate::*;

/// *power_expression* | `-` *power_expression*
pub(crate) fn unary_minus_expression(i: Input) -> AstResult {
    stub(i)
}

/// *primary_expression* | `~` *unary_expression* | `+` *unary_expression* | `!` *unary_expression*
pub(crate) fn unary_expression(i: Input) -> AstResult {
    stub(i)
}

fn stub(i: Input) -> AstResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
