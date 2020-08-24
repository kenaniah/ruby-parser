use crate::ast::Literal;
use crate::lexer::*;
use crate::parsers::token::literal::string::double::double_quoted_string;
use crate::parsers::token::literal::string::quoted::non_expanded_delimited_string;
use crate::parsers::token::literal::string::single::single_quoted_string;
use crate::parsers::token::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, recognize};
use nom::sequence::tuple;

/// *symbol_literal* | *dynamic_symbol*
pub(crate) fn symbol(i: Input) -> TokenResult {
    alt((symbol_literal, dynamic_symbol))(i)
}

/// `:` *symbol_name*
pub(crate) fn symbol_literal(i: Input) -> TokenResult {
    map(recognize(tuple((char(':'), symbol_name))), |s| {
        Token::Literal(Literal::Symbol((*s).to_owned()))
    })(i)
}

/// `:` *single_quoted_string*  | `:` *double_quoted_string* | `%s` *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn dynamic_symbol(i: Input) -> TokenResult {
    alt((
        map(tuple((char(':'), single_quoted_string)), |mut t| {
            t.1.insert(0, ':');
            Token::Literal(Literal::Symbol(t.1))
        }),
        map(tuple((char(':'), double_quoted_string)), |t| match t.1 {
            Interpolatable::String(mut s) => {
                s.insert(0, ':');
                Token::Literal(Literal::Symbol(s))
            }
            Interpolatable::Interpolated(mut vec) => {
                vec.insert(0, Token::Segment(":".to_owned()));
                Token::InterpolatedSymbol(vec)
            }
        }),
        map(
            tuple((tag("%s"), non_expanded_delimited_string)),
            |mut t| {
                t.1.insert(0, ':');
                Token::Literal(Literal::Symbol(t.1))
            },
        ),
    ))(i)
}

/// *identifier* | *operator* | *keyword*
pub(crate) fn symbol_name(i: Input) -> LexResult {
    alt((recognize(identifier), recognize(operator), keyword))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_symbol {
        ($a:expr, $b:expr) => {
            assert_ok!($a, Token::Literal(Literal::Symbol($b.to_owned())))
        };
    }
    macro_rules! assert_interpolated {
        ($a:expr, $b:expr) => {
            assert_ok!($a, Token::InterpolatedSymbol($b))
        };
    }

    #[test]
    fn test_symbol_literal() {
        use_parser!(symbol_literal);
        // Parse errors
        assert_err!(":");
        assert_err!("foo");
        assert_err!("::");
        assert_err!(":foo bar");
        assert_err!(":@");
        assert_err!(":@@");
        assert_err!(":$");
        assert_err!(":[");
        assert_err!(":====");
        assert_err!(":foo==");
        // Success cases
        assert_symbol!(":foo", ":foo");
        assert_symbol!(":_", ":_");
        assert_symbol!(":===", ":===");
        assert_symbol!(":!", ":!");
        assert_symbol!(":[]", ":[]");
        assert_symbol!(":foo=", ":foo=");
        assert_symbol!(":>=", ":>=");
        assert_symbol!(":if", ":if");
        assert_symbol!(":$glob", ":$glob");
        assert_symbol!(":@@v", ":@@v");
        assert_symbol!(":CONST", ":CONST");
        assert_symbol!(":😉😎", ":😉😎");
    }

    #[test]
    fn test_dynamic_symbol() {
        use_parser!(dynamic_symbol);
        // Parse errors
        assert_err!("''");
        assert_err!(":'");
        assert_err!(":'\"");
        assert_err!(":'foo bar''");
        // Success cases
        assert_symbol!(":''", ":");
        assert_symbol!(":\"\"", ":");
        assert_symbol!(":'foo #$bar'", ":foo #$bar");
        assert_symbol!(":'$123'", ":$123");
        assert_symbol!(":\"\\x00\"", ":\0");
        assert_symbol!(":\"foo\\nbar\"", ":foo\nbar");
        assert_symbol!("%s(foo #{2 + 4} bar)", ":foo #{2 + 4} bar");
        assert_interpolated!(
            ":\"foo#$bar\"",
            vec![
                Token::Segment(":".to_owned()),
                Token::Segment("foo".to_owned()),
                Token::GlobalVariableIdentifier("$bar".to_owned())
            ]
        );
    }
}
