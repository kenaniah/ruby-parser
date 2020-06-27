use crate::lexers::{identifier, keyword, numeric_literal, string_literal};
use crate::{Input, ParseResult, Token, TokenResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, one_of};
use nom::combinator::{map, recognize};
use nom::sequence::tuple;

/// *keyword* | *identifier* | *punctuator* | *operator* | *literal*
pub fn token(i: Input) -> TokenResult {
    alt((keyword, identifier, punctuator, operator, literal))(i)
}

/// *numeric_literal* | *string_literal* | *array_literal* | *regular_expression_literal* | *symbol*
pub(crate) fn literal(i: Input) -> TokenResult {
    alt((
        numeric_literal,
        string_literal,
        //array_literal,
        //regular_expression_literal,
        //symbol,
    ))(i)
}

pub(crate) fn punctuator(i: Input) -> TokenResult {
    let (i, res) = alt((
        tag("..."),
        tag(".."),
        tag("::"),
        tag("=>"),
        recognize(one_of("[](){},;:?")),
    ))(i)?;
    let token = match *res {
        "[" => Token::LeftBracket,
        "]" => Token::RightBracket,
        "(" => Token::LeftParen,
        ")" => Token::RightParen,
        "," => Token::Comma,
        ";" => Token::Semicolon,
        ":" => Token::Colon,
        "?" => Token::QuestionMark,
        ".." => Token::DotDot,
        "..." => Token::DotDotDot,
        "=>" => Token::Arrow,
        "::" => Token::DoubleColon,
        _ => unreachable!(),
    };
    Ok((i, token))
}

/// `!` | `!=` | `!~` | `&&` | `||` | *operator_method_name* | `=` | *assignment_operator*
pub(crate) fn operator(i: Input) -> TokenResult {
    map(
        recognize(alt((
            assignment_operator,
            operator_method_name,
            tag("="),
            tag("||"),
            tag("&&"),
            tag("!~"),
            tag("!="),
            tag("!"),
        ))),
        |s: Input| Token::Operator((*s).to_owned()),
    )(i)
}

/// `^` | `&` | `|` | `<=>` | `==` | `===` | `=~` | `>` | `>=` | `<` | `<=` | `<<` | `>>` | `+` | `-` | `*` | `/` | `%` | `**` | `~` | `+@` | `-@` | `[]` | `[]=` | ```
pub(crate) fn operator_method_name(i: Input) -> ParseResult {
    recognize(alt((
        tag("<=>"),
        tag("==="),
        tag("=="),
        tag("=~"),
        tag(">="),
        tag(">>"),
        tag("<="),
        tag("<<"),
        tag("**"),
        tag(">"),
        tag("<"),
        tag("^"),
        tag("&"),
        tag("|"),
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("%"),
    )))(i)
}

/// *assignment_operator_name* `=`
pub(crate) fn assignment_operator(i: Input) -> ParseResult {
    recognize(tuple((assignment_operator_name, char('='))))(i)
}

/// `&&` | `||` | `^` | `&` | `|` | `<<` | `>>` | `+` | `-` | `%` | `/` | `**`
pub(crate) fn assignment_operator_name(i: Input) -> ParseResult {
    alt((
        tag("&&"),
        tag("||"),
        tag("^"),
        tag("&"),
        tag("|"),
        tag("<<"),
        tag(">>"),
        tag("+"),
        tag("-"),
        tag("%"),
        tag("/"),
        tag("**"),
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_token() {
        use_parser!(token, Input, Token, ErrorKind);
        // Identifiers
        assert_ok!("東", Token::LocalVariableIdentifier("東".to_owned()));
        assert_ok!("@_", Token::InstanceVariableIdentifier("@_".to_owned()));
        assert_ok!("$foo", Token::GlobalVariableIdentifier("$foo".to_owned()));
        // Punctuation
        assert_ok!("[", Token::LeftBracket);
        assert_ok!("[", Token::LeftBracket);
        assert_ok!("]", Token::RightBracket);
        assert_ok!("(", Token::LeftParen);
        assert_ok!(")", Token::RightParen);
        assert_ok!(",", Token::Comma);
        assert_ok!(";", Token::Semicolon);
        assert_ok!(":", Token::Colon);
        assert_ok!("?", Token::QuestionMark);
        assert_ok!("..", Token::DotDot);
        assert_ok!("...", Token::DotDotDot);
        assert_ok!("::", Token::DoubleColon);
        assert_ok!("=>", Token::Arrow);
    }
}
