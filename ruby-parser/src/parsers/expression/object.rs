use crate::lexer::*;
use crate::parsers::expression::operator_expression;

/// `[` *indexing_argument_list*? `]`
pub(crate) fn array_constructor(i: Input) -> NodeResult {
    stub(i)
}

/// `{` ( *association_list* [ no line terminator here ] `,`? )? `}`
pub(crate) fn hash_constructor(i: Input) -> NodeResult {
    stub(i)
}

/// *association* ( [ no line terminator here ] `,` *association* )*
pub(crate) fn association_list(i: Input) -> NodeResult {
    stub(i)
}

/// *association_key* [ no line terminator here ] `=>` *association_value*
pub(crate) fn association(i: Input) -> NodeResult {
    stub(i)
}

/// *operator_expression*
pub(crate) fn association_key(i: Input) -> NodeResult {
    operator_expression(i)
}

/// *operator_expression*
pub(crate) fn association_value(i: Input) -> NodeResult {
    operator_expression(i)
}

/// *operator_or_expression* | *operator_or_expression* [ no line terminator here ] *range_operator* *operator_or_expression*
pub(crate) fn range_constructor(i: Input) -> NodeResult {
    stub(i)
}

/// `..` | `...`
pub(crate) fn range_operator(i: Input) -> NodeResult {
    stub(i)
}

fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}
