use crate::lexers::token::literal;
use crate::{ExpressionResult, Input};
use nom::branch::alt;
use nom::combinator::map;

mod variable;

/// *class_definition* | *singleton_class_definition* | *module_definition* | *method_definition* | *singleton_method_definition* | *yield_with_optional_argument* | *if_expression* | *unless_expression* | *case_expression* | *while_expression* | *until_expression* | *for_expression* | *return_without_argument* | *break_without_argument* | *next_without_argument* | *redo_expression* | *retry_expression* | *begin_expression* | *grouping_expression* | *variable_reference* | *scoped_constant_reference* | *array_constructor* | *hash_constructor* | *literal* | *defined_with_parenthesis* | *primary_method_invocation*
/// NOTE: This was referred to as *primary-expression* in the ISO spec
pub fn expression(i: Input) -> ExpressionResult {
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
        //grouping_expression,
        map(variable::variable_reference, |t| vec![t]),
        //scoped_constant_reference,
        //array_constructor,
        //hash_constructor,
        map(literal, |t| vec![t]),
        //defined_with_parenthesis,
        //primary_method_invocation,
        stub,
    ))(i)
}

fn stub(i: Input) -> ExpressionResult {
    Err(nom::Err::Error((i, crate::ErrorKind::Char)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Token;

    #[test]
    fn test_expression() {
        use_parser!(expression, Input, Vec<Token>);
        // Parse errors
        assert_err!("");
        assert_err!("nil ");
        assert_err!("bar\n");
        // Success cases
        assert_ok!("nil", vec![Token::Nil]);
        assert_ok!("42", vec![Token::Integer(42)]);
        assert_ok!("24.2", vec![Token::Float(24.2)]);
        assert_ok!("meh", vec![Token::LocalVariableIdentifier("meh".to_owned())]);
        assert_ok!("-23e4", vec![Token::Float(-230000.0)]);
        assert_ok!("'hello world'", vec![Token::SingleQuotedString("hello world".to_owned())]);
    }
}
