use crate::*;

/// *keyword_not_expression* | *keyword_and_expression* | *keyword_or_expression*
pub(crate) fn keyword_logical_expression(i: Input) -> AstResult {
    stub(i)
}

/// *method_invocation_without_parenthesis* | *operator_expression* | `!` *method_invocation_without_parenthesis* | `not` *keyword_not_expression*
pub(crate) fn keyword_not_expression(i: Input) -> AstResult {
    stub(i)
}

/// `!` ( *method_invocation_without_parenthesis* | *unary_expression* )
pub(crate) fn operator_not_expression(i: Input) -> AstResult {
    stub(i)
}

/// *expression* [ no line terminator here ] `and` *keyword_not_expression*
pub(crate) fn keyword_and_expression(i: Input) -> AstResult {
    stub(i)
}

/// *equality_expression* | *operator_and_expression* [ no line terminator here ] `&&` *equality_expression*
pub(crate) fn operator_and_expression(i: Input) -> AstResult {
    stub(i)
}

/// *expression* [ no line terminator here ] `or` *keyword_not_expression*
pub(crate) fn keyword_or_expression(i: Input) -> AstResult {
    stub(i)
}

/// *operator_and_expression* | *operator_or_expression* [ no line terminator here ] `||` *operator_and_expression*
pub(crate) fn operator_or_expression(i: Input) -> AstResult {
    stub(i)
}

fn stub(i: Input) -> AstResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
