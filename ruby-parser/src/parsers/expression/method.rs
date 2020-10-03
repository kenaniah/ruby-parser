use crate::ast::Parameter;
use crate::lexer::*;
use crate::parsers::expression::argument::argument_with_parenthesis;
use crate::parsers::expression::argument::argument_without_parenthesis;
use crate::parsers::expression::argument::comma;
use crate::parsers::expression::argument::indexing_argument_list;
use crate::parsers::expression::begin::body_statement;
use crate::parsers::expression::block::block;
use crate::parsers::expression::block::do_block;
use crate::parsers::expression::jump::{
    break_with_argument, next_with_argument, return_with_argument,
};
use crate::parsers::expression::operator_expression;
use crate::parsers::expression::primary_expression;
use crate::parsers::expression::recursing_primary_expression;
use crate::parsers::expression::super_::super_with_argument;
use crate::parsers::expression::super_::super_with_argument_and_do_block;
use crate::parsers::expression::yield_::yield_with_argument;
use crate::parsers::program::separator;
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
    alt((
        map(
            tuple((
                chained_command_with_do_block,
                alt((tag("."), tag("::"))),
                method_name,
                argument_without_parenthesis,
            )),
            |_| Node::Placeholder,
        ),
        chained_command_with_do_block,
        command,
        return_with_argument,
        break_with_argument,
        next_with_argument,
    ))(i)
}

/// *super_with_argument* | *yield_with_argument* | *method_identifier* *argument_without_parenthesis* | *primary_expression* [ no ⏎ ] ( `.` | `::` ) *method_name* *argument_without_parenthesis*
pub(crate) fn command(i: Input) -> NodeResult {
    alt((
        super_with_argument,
        yield_with_argument,
        map(
            tuple((method_identifier, argument_without_parenthesis)),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                alt((tag("."), tag("::"))),
                method_name,
                argument_without_parenthesis,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// *command_with_do_block* *chained_method_invocation**
pub(crate) fn chained_command_with_do_block(i: Input) -> NodeResult {
    map(
        tuple((command_with_do_block, chained_method_invocation)),
        |_| Node::Placeholder,
    )(i)
}

/// ( `.` | `::` ) *method_name* | ( `.` | `::` ) *method_name* [ no ⏎ ] [ no ⎵ ] *argument_with_parenthesis*
pub(crate) fn chained_method_invocation(i: Input) -> NodeResult {
    map(
        tuple((
            alt((tag("."), tag("::"))),
            method_name,
            opt(argument_with_parenthesis),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *super_with_argument_and_do_block* | *method_identifier* *argument_without_parenthesis* *do_block* | *primary_expression* [ no ⏎ ] ( `.` | `::` ) *method_name* *argument_without_parenthesis* *do_block*
pub(crate) fn command_with_do_block(i: Input) -> NodeResult {
    alt((
        super_with_argument_and_do_block,
        map(
            tuple((method_identifier, argument_without_parenthesis, do_block)),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                primary_expression,
                alt((tag("."), tag("::"))),
                method_name,
                argument_without_parenthesis,
                do_block,
            )),
            |_| Node::Placeholder,
        ),
    ))(i)
}

/// `(` *parameter_list*? `)` | *parameter_list*? *separator*
pub(crate) fn method_parameter_part(i: Input) -> NodeResult {
    alt((
        map(tuple((char('('), opt(parameter_list), char(')'))), |_| {
            Node::Placeholder
        }),
        map(tuple((opt(parameter_list), separator)), |_| {
            Node::Placeholder
        }),
    ))(i)
}

/// *mandatory_parameter_list* ( `,` *optional_parameter_list* )? ( `,` *array_parameter* )? ( `,` *proc_parameter* )? |  *optional_parameter_list* ( `,` *array_parameter* )? ( `,` *proc_parameter* )? | *array_parameter* ( `,` *proc_parameter* )? | *proc_parameter*
pub(crate) fn parameter_list(i: Input) -> NodeResult {
    alt((
        map(
            tuple((
                mandatory_parameter_list,
                opt(tuple((comma, ws0, optional_parameter_list))),
                opt(tuple((comma, ws0, array_parameter))),
                opt(tuple((comma, ws0, proc_parameter))),
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((
                optional_parameter_list,
                opt(tuple((comma, ws0, array_parameter))),
                opt(tuple((comma, ws0, proc_parameter))),
            )),
            |_| Node::Placeholder,
        ),
        map(
            tuple((array_parameter, opt(tuple((comma, ws0, proc_parameter))))),
            |_| Node::Placeholder,
        ),
        map(proc_parameter, |_| Node::Placeholder),
    ))(i)
}

/// *mandatory_parameter* | *mandatory_parameter_list* `,` *mandatory_parameter*
pub(crate) fn mandatory_parameter_list(i: Input) -> NodeResult {
    map(
        tuple((mandatory_parameter, opt(recursing_mandatory_parameter_list))),
        Node::decurse,
    )(i)
}

fn recursing_mandatory_parameter_list(i: Input) -> NodeResult {
    map(
        tuple((
            comma,
            ws0,
            mandatory_parameter,
            opt(recursing_mandatory_parameter_list),
        )),
        |_| Node::Placeholder,
    )(i)
}

/// *local_variable_identifier*
pub(crate) fn mandatory_parameter(i: Input) -> NodeResult {
    map(local_variable_identifier, |ident| Node::Identifier(ident))(i)
}

/// *optional_parameter* ( [ no ⏎ ] `,` *optional_parameter* )*
pub(crate) fn optional_parameter_list(i: Input) -> ParameterListResult {
    map(
        tuple((
            optional_parameter,
            many0(map(tuple((comma, ws0, optional_parameter)), |t| t.2)),
        )),
        |(first, mut vec)| {
            vec.insert(0, first);
            vec
        },
    )(i)
}

/// *optional_parameter_name* `=` *default_parameter_expression*
pub(crate) fn optional_parameter(i: Input) -> ParameterResult {
    map(
        tuple((
            optional_parameter_name,
            no_lt,
            char('='),
            ws0,
            default_parameter_expression,
        )),
        |t| Parameter {
            name: t.0.to_string(),
            default_value: Some(Box::new(t.4)),
        },
    )(i)
}

/// *local_variable_identifier*
pub(crate) fn optional_parameter_name(i: Input) -> LexResult {
    recognize(local_variable_identifier)(i)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinaryOpKind;

    #[test]
    fn test_optional_parameter() {
        use_parser!(optional_parameter);
        // Parse errors
        assert_err!("foo\n=1");
        assert_err!("Foo=1");
        assert_err!("bar=");
        // Success cases
        assert_ok!("foo=1", Parameter::new_with_default("foo", Node::int(1)));
        assert_ok!(
            "foo =\n1 + 2",
            Parameter::new_with_default(
                "foo",
                Node::binary_op(Node::int(1), BinaryOpKind::Add, Node::int(2))
            )
        );
    }
}
