use crate::lexer::*;
use crate::parsers::expression::argument::argument_with_parenthesis;
use crate::parsers::expression::argument::indexing_argument_list;
use crate::parsers::expression::begin::body_statement;
use crate::parsers::expression::block::block;
use crate::parsers::expression::operator_expression;
use crate::parsers::expression::recursing_primary_expression;
use crate::parsers::token::identifier::{
    assignment_like_method_identifier, constant_identifier, local_variable_identifier,
    method_only_identifier,
};
use crate::parsers::token::keyword::keyword;
use crate::parsers::token::operator::operator_method_name;

/// `def` *defined_method_name* [ no ⏎ ] *method_parameter_part* *method_body* `end`
pub(crate) fn method_definition(i: Input) -> NodeResult {
    map(
        tuple((
            tag("def"),
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

/// *method_name* | *assignment_like_method_identifier*
pub(crate) fn defined_method_name(i: Input) -> IdentifierResult {
    alt((method_name, assignment_like_method_identifier))(i)
}

/// *body_statement*
pub(crate) fn method_body(i: Input) -> NodeResult {
    body_statement(i)
}

/// *primary_expression* [ no ⏎ ] `.` *method_name* ( [ no ⏎ ] [ no ⎵ ] *argument_with_parenthesis* )? *block*? | *primary_expression* [ no ⏎ ] `::` *method_name* [ no ⏎ ] [ no ⎵ ] *argument_with_parenthesis* *block*? | *primary_expression* [ no ⏎ ] `::` *method_name_except_constant* *block*?
pub(crate) fn _primary_method_invocation(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                no_lt,
                char('.'),
                ws0,
                method_name,
                opt(argument_with_parenthesis),
                opt(block),
                opt(recursing_primary_expression),
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                no_lt,
                tag("::"),
                ws0,
                method_name,
                argument_with_parenthesis,
                opt(recursing_primary_expression),
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                no_lt,
                tag("::"),
                method_name_except_constant,
                ws0,
                opt(block),
                opt(recursing_primary_expression),
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *method_only_identifier*
pub(crate) fn method_only_invocation(i: Input) -> NodeResult {
    map(method_only_identifier, |_| Node::Placeholder)(i)
}

/// *method_identifier* *block*
pub(crate) fn method_invocation_with_block(i: Input) -> NodeResult {
    map(tuple((method_identifier, no_lt, block)), |_| {
        Node::Placeholder
    })(i)
}

/// *method_identifier* [ no ⏎ ] [ no ⎵ ] *argument_with_parenthesis* *block*?
pub(crate) fn method_invocation_with_parenthesis(i: Input) -> NodeResult {
    map(
        tuple((method_identifier, argument_with_parenthesis, opt(block))),
        |_| Node::Placeholder,
    )(i)
}

/// *local_variable_identifier* | *constant_identifier* | *method_only_identifier*
pub(crate) fn method_identifier(i: Input) -> IdentifierResult {
    alt((
        method_only_identifier,
        local_variable_identifier,
        constant_identifier,
    ))(i)
}

/// *method_identifier* | *operator_method_name* | *keyword*
pub(crate) fn method_name(i: Input) -> IdentifierResult {
    alt((
        method_identifier,
        map(operator_method_name, |s| {
            Identifier::new(s.to_string(), IdentifierKind::Method)
        }),
        map(keyword, |s| {
            Identifier::new(s.to_string(), IdentifierKind::Method)
        }),
    ))(i)
}

/// *primary_expression* [ no ⏎ ] [ no ⎵ ] `[` *indexing_argument_list*? `]`
pub(crate) fn _indexing_method_invocation(i: Input) -> NodeResult {
    map(
        tuple((
            char('['),
            ws0,
            opt(indexing_argument_list),
            ws0,
            char(']'),
            opt(recursing_primary_expression),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *method_name* **but not** *constant_identifier*
pub(crate) fn method_name_except_constant(i: Input) -> IdentifierResult {
    let (i, _) = peek(not(constant_identifier))(i)?;
    method_name(i)
}

/// *command* | *chained_command_with_do_block* | *chained_command_with_do_block* ( `.` | `::` ) *method_name* *argument_without_parenthesis* | *return_with_argument* | *break_with_argument* | *next_with_argument*
pub(crate) fn method_invocation_without_parenthesis(i: Input) -> NodeResult {
    stub(i)
}

/// *super_with_argument* | *yield_with_argument* | *method_identifier* *argument_without_parenthesis* | *primary_expression* [ no ⏎ ] ( `.` | `::` ) *method_name* *argument_without_parenthesis*
pub(crate) fn command(i: Input) -> NodeResult {
    stub(i)
}

/// *command_with_do_block* *chained_method_invocation**
pub(crate) fn chained_command_with_do_block(i: Input) -> NodeResult {
    stub(i)
}

/// ( `.` | `::` ) *method_name* | ( `.` | `::` ) *method_name* [ no ⏎ ] [ no ⎵ ] *argument_with_parenthesis*
pub(crate) fn chained_method_invocation(i: Input) -> NodeResult {
    stub(i)
}

/// *super_with_argument_and_do_block* | *method_identifier* *argument_without_parenthesis* *do_block* | *primary_expression* [ no ⏎ ] ( `.` | `::` ) *method_name* *argument_without_parenthesis* *do_block*
pub(crate) fn command_with_do_block(i: Input) -> NodeResult {
    stub(i)
}

/// `(` *parameter_list*? `)` | *parameter_list*? *separator*
pub(crate) fn method_parameter_part(i: Input) -> NodeResult {
    stub(i)
}

/// *mandatory_parameter_list* ( `,` *optional_parameter_list* )? ( `,` *array_parameter* )? ( `,` *proc_parameter* )? |  *optional_parameter_list* ( `,` *array_parameter* )? ( `,` *proc_parameter* )? | *array_parameter* ( `,` *proc_parameter* )? | *proc_parameter*
pub(crate) fn parameter_list(i: Input) -> NodeResult {
    stub(i)
}

/// *mandatory_parameter* | *mandatory_parameter_list* `,` *mandatory_parameter*
pub(crate) fn mandatory_parameter_list(i: Input) -> NodeResult {
    stub(i)
}

/// *local_variable_identifier*
pub(crate) fn mandatory_parameter(i: Input) -> IdentifierResult {
    local_variable_identifier(i)
}

/// *optional_parameter* | *optional_parameter_list* `,` *optional_parameter*
pub(crate) fn optional_parameter_list(i: Input) -> NodeResult {
    stub(i)
}

/// *optional_parameter_name* `=` *default_parameter_expression*
pub(crate) fn optional_parameter(i: Input) -> NodeResult {
    stub(i)
}

/// *local_variable_identifier*
pub(crate) fn optional_parameter_name(i: Input) -> IdentifierResult {
    local_variable_identifier(i)
}

/// *operator_expression*
pub(crate) fn default_parameter_expression(i: Input) -> NodeResult {
    operator_expression(i)
}

/// `*` *array_parameter_name* | `*`
pub(crate) fn array_parameter(i: Input) -> NodeResult {
    map(preceded(char('*'), opt(array_parameter_name)), |n| {
        if let Some(ident) = n {
            Node::Splat(Box::new(Node::Identifier(ident)))
        } else {
            Node::Splat(Box::new(Node::None))
        }
    })(i)
}

/// *local_variable_identifier*
pub(crate) fn array_parameter_name(i: Input) -> IdentifierResult {
    local_variable_identifier(i)
}

/// `&` *proc_parameter_name*
pub(crate) fn proc_parameter(i: Input) -> IdentifierResult {
    map(tuple((char('&'), ws0, proc_parameter_name)), |t| t.2)(i)
}

/// *local_variable_identifier*
pub(crate) fn proc_parameter_name(i: Input) -> IdentifierResult {
    local_variable_identifier(i)
}
