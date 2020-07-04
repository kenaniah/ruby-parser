use crate::{ExpressionResult, Input};
use nom::branch::alt;
use nom::combinator::map;

mod variable;

/// *class_definition* | *singleton_class_definition* | *module_definition* | *method_definition* | *singleton_method_definition* | *yield_with_optional_argument* | *if_expression* | *unless_expression* | *case_expression* | *while_expression* | *until_expression* | *for_expression* | *return_without_argument* | *break_without_argument* | *next_without_argument* | *redo_expression* | *retry_expression* | *begin_expression* | *grouping_expression* | *variable_reference* | *scoped_constant_reference* | *array_constructor* | *hash_constructor* | *literal* | *defined_with_parenthesis* | *primary_method_invocation*
/// NOTE: This was referred to as *primary-expression* in the ISO spec
pub fn expression(i: Input) -> ExpressionResult {
    alt((map(variable::variable_reference, |t| vec![t]), stub))(i)
}

fn stub(i: Input) -> ExpressionResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Char)))
}
