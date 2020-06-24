use crate::{Input, TokenResult};

/// `__LINE__` | `__ENCODING__` | `__FILE__` | `BEGIN` | `END` | `alias` | `and` | `begin` | `break` | `case` | `class` | `def` | `defined?` | `do` | `else` | `elsif` | `end` | `ensure` | `for` | `false` | `if` | `in` | `module` | `next` | `nil` | `not` | `or` | `redo` | `rescue` | `retry` | `return` | `self` | `super` | `then` | `true` | `undef` | `unless` | `until` | `when` | `while` | `yield`
pub fn keyword(i: Input) -> TokenResult {
    stub_token(i)
}

fn stub_token(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Complete)))
}
