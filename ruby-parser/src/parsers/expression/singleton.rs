use crate::lexer::*;
use crate::parsers::expression::begin::body_statement;
use crate::parsers::expression::expression;
use crate::parsers::expression::method::defined_method_name;
use crate::parsers::expression::method::method_body;
use crate::parsers::expression::method::method_parameter_part;
use crate::parsers::expression::variable::variable_reference;
use crate::parsers::program::separator;

/// `class` `<<` *expression* *separator* *singleton_class_body* `end`
pub(crate) fn singleton_class_definition(i: Input) -> NodeResult {
    map(
        tuple((
            tag("class"),
            ws0,
            tag("<<"),
            ws0,
            expression,
            separator,
            singleton_class_body,
            tag("end"),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *body_statement*
pub(crate) fn singleton_class_body(i: Input) -> NodeResult {
    body_statement(i)
}

/// `def` *singleton* ( `.` | `::` ) *defined_method_name* [ no ⏎ ] *method_parameter_part* *method_body* `end`
pub(crate) fn singleton_method_definition(i: Input) -> NodeResult {
    map(
        tuple((
            tag("def"),
            ws0,
            singleton,
            alt((tag("."), tag("::"))),
            ws0,
            defined_method_name,
            no_lt,
            method_parameter_part,
            method_body,
            tag("end"),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *variable_reference* | `(` *expression* `)`
pub(crate) fn singleton(i: Input) -> NodeResult {
    alt((
        variable_reference,
        map(tuple((char('('), ws0, expression, ws0, char(')'))), |t| t.2),
    ))(i)
}
