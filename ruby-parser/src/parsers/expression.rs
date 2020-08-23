use crate::parsers::program::compound_statement;
use crate::parsers::token::literal;
use crate::*;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::tuple;

mod argument;
mod assignment;
mod binary;
mod block;
mod logical;
mod method;
mod super_;
mod unary;
mod variable;
mod yield_;

pub(crate) fn expression(i: Input) -> TokenResult {
    logical::keyword_logical_expression(i)
}

/// *class_definition* | *singleton_class_definition* | *module_definition* | *method_definition* | *singleton_method_definition* | *yield_with_optional_argument* | *if_expression* | *unless_expression* | *case_expression* | *while_expression* | *until_expression* | *for_expression* | *return_without_argument* | *break_without_argument* | *next_without_argument* | *redo_expression* | *retry_expression* | *begin_expression* | *grouping_expression* | *variable_reference* | *scoped_constant_reference* | *array_constructor* | *hash_constructor* | *literal* | *defined_with_parenthesis* | *primary_method_invocation*
pub(crate) fn primary_expression(i: Input) -> TokenResult {
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
pub(crate) fn grouping_expression(i: Input) -> TokenResult {
    map(tuple((char('('), compound_statement, char(')'))), |t| t.1)(i)
}

/// *assignment_expression* | *defined_without_parenthesis* | *conditional_operator_expression*
pub(crate) fn operator_expression(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;

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
        assert_ok!("nil", Token::Nil);
        assert_ok!("42", Token::Integer(42));
        assert_ok!("24.2", Token::Float(24.2));
        assert_ok!("meh", Token::LocalVariableIdentifier("meh".to_owned()));
        assert_ok!("-23e4", Token::Float(-230000.0));
        assert_ok!("'hello world'", Token::String("hello world".to_owned()));
        assert_ok!("()", Token::Block(vec![]));
        assert_ok!(
            "((false))",
            Token::Block(vec![Token::Block(vec![Token::False])])
        );
        assert_ok!(
            "(;2\n\t5;;)",
            Token::Block(vec![Token::Integer(2), Token::Integer(5)])
        );
        assert_ok!("(;)", Token::Block(vec![]));
    }
}
