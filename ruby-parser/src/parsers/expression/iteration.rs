use crate::lexer::*;

/// `while` *expression* *do_clause* `end`
pub(crate) fn while_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *separator* *compound_statement* | [ no line terminator here ] `do` *compound_statement*
pub(crate) fn do_clause(i: Input) -> NodeResult {
    stub(i)
}

/// `until` *expression* *do_clause* `end`
pub(crate) fn until_expression(i: Input) -> NodeResult {
    stub(i)
}

/// `for` *for_variable* [ no line terminator here ] `in` *expression* *do_clause* `end`
pub(crate) fn for_expression(i: Input) -> NodeResult {
    stub(i)
}

/// *left_hand_side* | *multiple_left_hand_side*
pub(crate) fn for_variable(i: Input) -> NodeResult {
    stub(i)
}
