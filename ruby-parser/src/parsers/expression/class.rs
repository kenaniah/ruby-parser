use crate::lexer::*;
use crate::parsers::expression::expression;
use crate::parsers::expression::module::{module_body, module_path};

/// `class` *module_path* [ no line terminator here ] ( `<` *superclass* )? *module_body* `end`
pub(crate) fn class_definition(i: Input) -> NodeResult {
    map(
        tuple((
            tag("class"),
            module_path,
            no_lt,
            opt(tuple((char('<'), ws, superclass))),
            module_body,
            ws,
            tag("end"),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *expression*
pub(crate) fn superclass(i: Input) -> NodeResult {
    expression(i)
}
