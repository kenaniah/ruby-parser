use crate::lexer::*;
use crate::parsers::expression::object::association;
use crate::parsers::expression::object::range_constructor;
use crate::parsers::program::compound_statement;
use crate::parsers::token::literal::literal;

pub(crate) mod argument;
pub(crate) mod assignment;
pub(crate) mod begin;
pub(crate) mod binary;
pub(crate) mod block;
pub(crate) mod class;
pub(crate) mod conditional;
pub(crate) mod defined;
pub(crate) mod iteration;
pub(crate) mod jump;
pub(crate) mod logical;
pub(crate) mod method;
pub(crate) mod module;
pub(crate) mod object;
pub(crate) mod super_;
pub(crate) mod unary;
pub(crate) mod variable;
pub(crate) mod yield_;

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
        conditional::if_expression,
        conditional::unless_expression,
        conditional::case_expression,
        iteration::while_expression,
        iteration::until_expression,
        //for_expression,
        jump::return_without_argument,
        jump::break_without_argument,
        jump::next_without_argument,
        jump::redo_expression,
        jump::retry_expression,
        //begin_expression,
        grouping_expression,
        variable::variable_reference,
        //scoped_constant_reference,
        object::array_constructor,
        object::hash_constructor,
        literal,
        defined::defined_with_parenthesis,
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
        defined::defined_without_parenthesis,
        conditional::conditional_operator_expression,
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_ok!("42", Node::int(42));
        assert_ok!("24.2", Node::float(24.2));
        assert_ok!("meh", Node::ident("meh", IdentifierKind::LocalVariable));
        assert_ok!("-23e4", Node::float(-230000.0));
        assert_ok!("'hello world'", Node::literal_string("hello world"));
        assert_ok!("redo", Node::Redo);
        assert_ok!("retry", Node::Retry);
        assert_ok!("return", Node::Return(vec![]));
        assert_ok!("break", Node::Break(vec![]));
        assert_ok!("next", Node::Next(vec![]));
        assert_ok!("()", Node::Block(vec![]));
        assert_ok!(
            "((false))",
            Node::Block(vec![Node::Block(vec![Node::boolean(false)])])
        );
        assert_ok!("(;2\n\t5;;)", Node::Block(vec![Node::int(2), Node::int(5)]));
        assert_ok!("(;)", Node::Block(vec![]));
    }
}
