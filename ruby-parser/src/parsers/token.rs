use crate::lexer::*;
use nom::branch::alt;
use nom::combinator::map;

pub(crate) mod identifier;
pub(crate) mod keyword;
pub(crate) mod literal;
pub(crate) mod operator;
pub(crate) mod punctuator;

pub(crate) use identifier::identifier;
pub(crate) use keyword::keyword;
pub(crate) use literal::literal;
pub(crate) use operator::operator;
pub(crate) use punctuator::punctuator;

/// *keyword* | *identifier* | *punctuator* | *operator* | *literal*
pub(crate) fn token(i: Input) -> TokenResult {
    alt((
        map(keyword, |s| Token::Keyword((*s).to_owned())),
        identifier,
        punctuator,
        operator,
        literal,
    ))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        use_parser!(token);
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
