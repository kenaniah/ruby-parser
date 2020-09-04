use crate::lexer::*;

/// *brace_block* | *do block*
pub(crate) fn block(i: Input) -> NodeResult {
    stub(i)
}

/// `{` *block_parameter*? *block_body* }
pub(crate) fn brace_block(i: Input) -> NodeResult {
    stub(i)
}

/// `do` *block_parameter*? *block_body* `end`
pub(crate) fn do_block(i: Input) -> NodeResult {
    stub(i)
}

/// `| |` | `||` | `|` *block_parameter_list* `|`
pub(crate) fn block_parameter(i: Input) -> NodeResult {
    stub(i)
}

/// *left_hand_side* | *multiple_left_hand_side*
pub(crate) fn block_parameter_list(i: Input) -> NodeResult {
    stub(i)
}

/// *compound_statement*
pub(crate) fn block_body(i: Input) -> NodeResult {
    stub(i)
}
