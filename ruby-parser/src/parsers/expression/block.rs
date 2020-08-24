use crate::lexer::*;

/// *brace_block* | *do block*
pub(crate) fn block(i: Input) -> TokenResult {
    stub(i)
}

/// `{` *block_parameter*? *block_body* }
pub(crate) fn brace_block(i: Input) -> TokenResult {
    stub(i)
}

/// `do` *block_parameter*? *block_body* `end`
pub(crate) fn do_block(i: Input) -> TokenResult {
    stub(i)
}

/// `| |` | `||` | `|` *block_parameter_list* `|`
pub(crate) fn block_parameter(i: Input) -> TokenResult {
    stub(i)
}

/// *left_hand_side* | *multiple_left_hand_side*
pub(crate) fn block_parameter_list(i: Input) -> TokenResult {
    stub(i)
}

/// *compound_statement*
pub(crate) fn block_body(i: Input) -> TokenResult {
    stub(i)
}

// /// docs
// pub(crate) fn func(i: Input) -> TokenResult {
//     stub(i)
// }

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
