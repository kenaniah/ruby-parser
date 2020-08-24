use crate::lexer::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, recognize};
use nom::sequence::tuple;

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

/// `^` | `&` | `|` | `<=>` | `==` | `===` | `=~` | `>` | `>=` | `<` | `<=` | `<<` | `>>` | `+` | `-` | `*` | `/` | `%` | `**` | `~` | `+@` | `-@` | `[]` | `[]=`
pub(crate) fn operator_method_name(i: Input) -> LexResult {
    recognize(alt((
        alt((
            tag("<=>"),
            tag("==="),
            tag("[]="),
            tag("=="),
            tag("=~"),
            tag(">="),
            tag(">>"),
            tag("<="),
            tag("<<"),
            tag("**"),
            tag(">"),
            tag("<"),
        )),
        alt((
            tag("^"),
            tag("&"),
            tag("|"),
            tag("+"),
            tag("-"),
            tag("*"),
            tag("/"),
            tag("%"),
            tag("~"),
            tag("+@"),
            tag("-@"),
            tag("[]"),
        )),
    )))(i)
}

/// *assignment_operator_name* `=`
pub(crate) fn assignment_operator(i: Input) -> LexResult {
    recognize(tuple((assignment_operator_name, char('='))))(i)
}

/// `&&` | `||` | `^` | `&` | `|` | `<<` | `>>` | `+` | `-` | `%` | `/` | `**`
pub(crate) fn assignment_operator_name(i: Input) -> LexResult {
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
