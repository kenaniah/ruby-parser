use crate::lexer::*;
use crate::parsers::expression::expression;
use crate::parsers::expression::module::{module_body, module_path};

/// `class` *module_path* [ no ⏎ ] ( `<` *superclass* )? *module_body* `end`
pub(crate) fn class_definition(i: Input) -> NodeResult {
    map(
        tuple((
            tag("class"),
            ws0,
            module_path,
            no_lt,
            opt(tuple((char('<'), ws0, superclass))),
            module_body,
            tag("end"),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *expression*
pub(crate) fn superclass(i: Input) -> NodeResult {
    expression(i)
}
