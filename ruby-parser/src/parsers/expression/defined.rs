use crate::lexer::*;

/// `defined?` `(` *expression* `)`
pub(crate) fn defined_with_parenthesis(i: Input) -> NodeResult {
    stub(i)
}

/// `defined?` *operator_expression*
pub(crate) fn defined_without_parenthesis(i: Input) -> NodeResult {
    stub(i)
}
