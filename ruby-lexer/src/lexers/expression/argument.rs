use crate::*;

/// *command* | *operator_expression_list* ( [ no line terminator here ] `,` )? | *operator_expression_list* ( [ no line terminator here ] `,` *splatting_argument* ) | *association_list* ( [ no line terminator here ] `,` )? | *splatting_argument*
pub(crate) fn indexing_argument_list(i: Input) -> TokenResult {
    stub(i)
}

/// `*` *operator_expression*
pub(crate) fn splatting_argument(i: Input) -> TokenResult {
    stub(i)
}

/// *operator_expression* ( [ no line terminator here ] `,` *operator_expression* )*
pub(crate) fn operator_expression_list(i: Input) -> TokenResult {
    stub(i)
}

/// `()` | `(` *argument_list* `)` | `(` *operator_expression_list* [ no line terminator here ] `,` *chained_command_with_do_block* `)` | `(` *chained_command_with_do_block* `)`
pub(crate) fn argument_with_parentheses(i: Input) -> TokenResult {
    stub(i)
}

/// *block_argument* | *splatting_argument* ( `,` *block_argument* )? | *operator_expression_list* [ no line terminator here ] `,` *association_list* ( [ no line terminator here ] `,` *splatting_argument* )? ( [ no line terminator here ] `,` *block_argument* )? | ( *operator_expression_list* | *association_list* ) ( [ no line terminator here ] `,` *splatting_argument* )? ( [no line terminator here ] `,` *block_argument* )? | *command*
pub(crate) fn argument_list(i: Input) -> TokenResult {
    stub(i)
}

/// `&` *operator_expression*
pub(crate) fn block_argument(i: Input) -> TokenResult {
    stub(i)
}

fn stub(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
