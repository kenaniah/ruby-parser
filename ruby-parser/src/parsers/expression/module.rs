use crate::lexer::*;
use crate::parsers::expression::begin::body_statement;
use crate::parsers::expression::primary_expression;
use crate::parsers::token::identifier::constant_identifier;

/// `module` *module_path* *module_body* `end`
pub(crate) fn module_definition(i: Input) -> NodeResult {
    map(
        tuple((tag("module"), ws0, module_path, module_body, tag("end"))),
        |_| Node::Placeholder,
    )(i)
}

/// *top_module_path* | *module_name* | *nested_module_path*
pub(crate) fn module_path(i: Input) -> NodeResult {
    alt((top_module_path, module_name, nested_module_path))(i)
}

/// *constant_identifier*
pub(crate) fn module_name(i: Input) -> NodeResult {
    map(constant_identifier, |v| Node::from(v))(i)
}

/// `::` *module_name*
pub(crate) fn top_module_path(i: Input) -> NodeResult {
    map(tuple((tag("::"), module_name)), |_| Node::Placeholder)(i)
}

/// *primary_expression* [ no ⏎ ] `::` *module_name*
pub(crate) fn nested_module_path(i: Input) -> NodeResult {
    map(
        tuple((primary_expression, no_lt, tag("::"), module_name)),
        |_| Node::Placeholder,
    )(i)
}

/// *body_statement*
pub(crate) fn module_body(i: Input) -> NodeResult {
    body_statement(i)
}
