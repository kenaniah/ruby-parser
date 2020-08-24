use crate::lexer::LexResult;
use crate::Input;
use nom::branch::alt;
use nom::bytes::complete::tag;

/// `__LINE__` | `__ENCODING__` | `__FILE__` | `BEGIN` | `END` | `alias` | `and` | `begin` | `break` | `case` | `class` | `def` | `defined?` | `do` | `else` | `elsif` | `end` | `ensure` | `for` | `false` | `if` | `in` | `module` | `next` | `nil` | `not` | `or` | `redo` | `rescue` | `retry` | `return` | `self` | `super` | `then` | `true` | `undef` | `unless` | `until` | `when` | `while` | `yield`
pub(crate) fn keyword(i: Input) -> LexResult {
    alt((
        alt((
            tag("alias"),
            tag("and"),
            tag("begin"),
            tag("break"),
            tag("case"),
            tag("class"),
            tag("def"),
            tag("defined?"),
            tag("do"),
            tag("else"),
            tag("elsif"),
            tag("end"),
            tag("ensure"),
        )),
        alt((
            tag("for"),
            tag("false"),
            tag("if"),
            tag("in"),
            tag("module"),
            tag("next"),
            tag("nil"),
            tag("not"),
            tag("or"),
            tag("redo"),
            tag("rescue"),
            tag("retry"),
            tag("return"),
            tag("self"),
            tag("super"),
            tag("then"),
            tag("true"),
        )),
        alt((
            tag("undef"),
            tag("unless"),
            tag("until"),
            tag("when"),
            tag("while"),
            tag("yield"),
            tag("__LINE__"),
            tag("__ENCODING__"),
            tag("__FILE__"),
            tag("BEGIN"),
            tag("END"),
        )),
    ))(i)
}
