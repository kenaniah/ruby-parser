use crate::ast::Conditional;
use crate::lexer::*;
use crate::parsers::expression::object::range_constructor;
use crate::parsers::program::{compound_statement, no_lt, ws};
use crate::parsers::token::literal::literal;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::sequence::tuple;

mod argument;
mod assignment;
mod binary;
mod block;
mod logical;
mod method;
mod object;
mod super_;
mod unary;
mod variable;
mod yield_;

pub(crate) fn expression(i: Input) -> NodeResult {
    logical::keyword_logical_expression(i)
}

/// *class_definition* | *singleton_class_definition* | *module_definition* | *method_definition* | *singleton_method_definition* | *yield_with_optional_argument* | *if_expression* | *unless_expression* | *case_expression* | *while_expression* | *until_expression* | *for_expression* | *return_without_argument* | *break_without_argument* | *next_without_argument* | *redo_expression* | *retry_expression* | *begin_expression* | *grouping_expression* | *variable_reference* | *scoped_constant_reference* | *array_constructor* | *hash_constructor* | *literal* | *defined_with_parenthesis* | *primary_method_invocation*
pub(crate) fn primary_expression(i: Input) -> NodeResult {
    let i = stack_frame!("primary_expression", i);
    alt((
        //class_definition,
        //singleton_class_definition,
        //module_definition,
        //method_definition,
        //singleton_method_definition,
        //yield_with_optional_argument,
        //if_expression,
        //unless_expression,
        //case_expression,
        //while_expression,
        //until_expression,
        //for_expression,
        //return_without_argument,
        //break_without_argument,
        //next_without_argument,
        //redo_expression,
        //retry_expression,
        //begin_expression,
        grouping_expression,
        variable::variable_reference,
        //scoped_constant_reference,
        //array_constructor,
        //hash_constructor,
        literal,
        //defined_with_parenthesis,
        //primary_method_invocation,
    ))(i)
}

/// `(` *compound_statement* `)`
pub(crate) fn grouping_expression(i: Input) -> NodeResult {
    map(tuple((char('('), compound_statement, char(')'))), |t| t.1)(i)
}

/// *assignment_expression* | *defined_without_parenthesis* | *conditional_operator_expression*
pub(crate) fn operator_expression(i: Input) -> NodeResult {
    alt((
        assignment::assignment_expression,
        defined_without_parenthesis,
        conditional_operator_expression,
    ))(i)
}

/// *range_constructor* | *range_constructor* [ no line terminator here ] `?` *operator_expression* [ no line terminator here ] `:` *operator_expression*
pub(crate) fn conditional_operator_expression(i: Input) -> NodeResult {
    let i = stack_frame!("conditional_operator_expression", i);
    map(
        tuple((range_constructor, opt(_conditional_operator_expression))), // BUG: range constructor is currently too greedy
        |(node, ast)| update_placeholder!(node, ast),
    )(i)
}

fn _conditional_operator_expression(i: Input) -> NodeResult {
    let i = stack_frame!("_conditional_operator_expression", i);
    alt((
        map(
            tuple((
                no_lt,
                char('?'),
                ws,
                operator_expression,
                no_lt,
                char(':'),
                ws,
                operator_expression,
                opt(_conditional_operator_expression),
            )),
            |t| {
                let node = Node::Conditional(Conditional {
                    cond: Box::new(Node::Placeholder),
                    then: Some(Box::new(t.3)),
                    otherwise: Some(Box::new(t.7)),
                });
                update_placeholder!(node, t.8)
            },
        ),
        range_constructor,
    ))(i)
}

/// `defined?` `(` *expression* `)`
pub(crate) fn defined_with_parenthesis(i: Input) -> NodeResult {
    stub(i)
}

/// `defined?` *operator_expression*
pub(crate) fn defined_without_parenthesis(i: Input) -> NodeResult {
    stub(i)
}

fn stub(i: Input) -> NodeResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_operator_expression() {
        use_parser!(conditional_operator_expression);
        // Parse errors
        assert_err!("");
        assert_err!("?2:3");
        assert_err!("1?:3");
        // Success cases
        assert_ok!("\"hi\"", Node::literal_string("hi"));
        let ok = Node::Conditional(Conditional {
            cond: Box::new(Node::integer(1)),
            then: Some(Box::new(Node::integer(2))),
            otherwise: Some(Box::new(Node::integer(3))),
        });
        assert_ok!("1 ? 2 : 3", ok);
        assert_ok!("1 ? 2: 3", ok);
        assert_ok!("1?2 : 3", ok);
        assert_ok!("1 ?2 :3", ok);
        assert_ok!("1 ? 2:3", ok);
        assert_ok!("1?2:3", ok);
        assert_ok!(
            "1 ? 2 ? 3 : 4 : 5",
            Node::Conditional(Conditional {
                cond: Box::new(Node::integer(1)),
                then: Some(Box::new(Node::Conditional(Conditional {
                    cond: Box::new(Node::integer(2)),
                    then: Some(Box::new(Node::integer(3))),
                    otherwise: Some(Box::new(Node::integer(4)))
                }))),
                otherwise: Some(Box::new(Node::integer(5)))
            })
        );
    }

    #[test]
    fn test_primary_expression() {
        use_parser!(primary_expression);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("bar\n");
        assert_err!("('");
        assert_err!("((foo)");
        // Success cases
        assert_ok!("nil", Node::Nil);
        assert_ok!("42", Node::integer(42));
        assert_ok!("24.2", Node::float(24.2));
        assert_ok!("meh", Node::ident("meh", IdentifierKind::LocalVariable));
        assert_ok!("-23e4", Node::float(-230000.0));
        assert_ok!("'hello world'", Node::literal_string("hello world"));
        assert_ok!("()", Node::Block(vec![]));
        assert_ok!(
            "((false))",
            Node::Block(vec![Node::Block(vec![Node::boolean(false)])])
        );
        assert_ok!(
            "(;2\n\t5;;)",
            Node::Block(vec![Node::integer(2), Node::integer(5)])
        );
        assert_ok!("(;)", Node::Block(vec![]));
    }
}
