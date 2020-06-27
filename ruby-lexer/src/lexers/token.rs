use crate::lexers::identifier;
use crate::{Input, Token, TokenResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::recognize;

/// *keyword* | *identifier* | *punctuator* | *operator* | *literal*
pub fn token(i: Input) -> TokenResult {
    alt((
        //keyword,
        identifier, punctuator,
        //operator,
        //literal
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
    unimplemented!()
}

/// `^` | `&` | `|` | `<=>` | `==` | `===` | `=~` | `>` | `>=` | `<` | `<=` | `<<` | `>>` | `+` | `-` | `*` | `/` | `%` | `**` | `~` | `+@` | `-@` | `[]` | `[]=` | ```
pub(crate) fn operator_method_name(i: Input) -> TokenResult {
    unimplemented!()
}

/// *assignment_operator_name* `=`
pub(crate) fn assignment_operator(i: Input) -> TokenResult {
    unimplemented!()
}

/// `&&` | `||` | `^` | `&` | `|` | `<<` | `>>` | `+` | `-` | `%` | `/` | `%` | `**`
pub(crate) fn assignment_operator_name(i: Input) -> TokenResult {
    unimplemented!()
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
