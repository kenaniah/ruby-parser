use crate::{CharResult, Input, StringResult, TokenResult};
use nom::character::complete::anychar;
use nom::combinator::{map, recognize, verify};
use nom::sequence::tuple;

/// *local_variable_identifier* | *global_variable_identifier* | *class_variable_identifier* | *instance_variable_identifier* | *constant_identifier* | *method_only_identifier* | *assignment_like_method_identifier*
pub fn identifier(i: Input) -> TokenResult {
    stub_token(i)
}

/// ( *lowercase_character* | `_` ) *identifier_character**
pub(crate) fn local_variable_identifier(i: Input) -> StringResult {
    stub_string(i)
}

/// `$` *identifier_start_character* *identifier_character**
pub(crate) fn global_variable_identifier(i: Input) -> StringResult {
    stub_string(i)
}

/// `@@` *identifier_start_character* *identifier_character**
pub(crate) fn class_variable_identifier(i: Input) -> StringResult {
    stub_string(i)
}

/// `@` *identifier_start_character* *identifier_character**
pub(crate) fn instance_variable_identifier(i: Input) -> StringResult {
    stub_string(i)
}

/// *uppercase_character* *identifier_character**
pub(crate) fn constant_identifier(i: Input) -> StringResult {
    map(
        recognize(tuple((title_case_character, identifier_character))),
        |s| (*s).to_owned(),
    )(i)
}

/// Returns any UTF-8 upper case or title case character
fn title_case_character(i: Input) -> CharResult {
    // Note: MRI also supports using unicode title case characters (in addition to uppercase)
    // rust currently does not have title case detection available in its internal unicode lib
    verify(anychar, |c: &char| c.is_uppercase())(i)
}

/// ( *constant_identifier* | *local_variable_identifier* ) ( `!` | `?` )
pub(crate) fn method_only_identifier(i: Input) -> StringResult {
    stub_string(i)
}

/// ( *constant_identifier* | *local_variable_identifier* ) `=`
pub(crate) fn assignment_like_method_identifier(i: Input) -> StringResult {
    stub_string(i)
}

/// *lowercase_character* | *uppercase_character* | *decimal_digit* | `_`
pub(crate) fn identifier_character(i: Input) -> CharResult {
    verify(anychar, |c: &char| {
        *c == '_' || c.is_ascii_alphanumeric() || !c.is_ascii()
    })(i)
}

/// *lowercase_character* | *uppercase_character* | `_`
pub(crate) fn identifier_start_character(i: Input) -> CharResult {
    verify(anychar, |c: &char| {
        *c == '_' || c.is_ascii_alphabetic() || !c.is_ascii()
    })(i)
}

fn stub_string(i: Input) -> StringResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Complete)))
}

fn stub_token(i: Input) -> TokenResult {
    Err(nom::Err::Error((i, nom::error::ErrorKind::Complete)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn test_identifier_character() {
        use_parser!(identifier_character, Input, char, ErrorKind);
        //Parse errors
        assert_err!("");
        assert_err!(" ");
        assert_err!("=");
        assert_err!("!");
        assert_err!("\n");
        assert_err!("\0");
        //Success cases
        assert_ok!("_");
        assert_ok!("a");
        assert_ok!("Z");
        assert_ok!("0");
        assert_ok!("9");
        assert_ok!("😀", '😀');
        assert_ok!("中", '中');
        assert_ok!("東", '東');
        assert_ok!("δ", 'δ');
        assert_ok!("Δ", 'Δ');
    }

    #[test]
    fn test_identifier_start_character() {
        use_parser!(identifier_start_character, Input, char, ErrorKind);
        //Parse errors
        assert_err!("");
        assert_err!(" ");
        assert_err!("=");
        assert_err!("!");
        assert_err!("0");
        assert_err!("9");
        assert_err!("\n");
        assert_err!("\0");
        //Success cases
        assert_ok!("_");
        assert_ok!("a");
        assert_ok!("Z");
        assert_ok!("😀", '😀');
        assert_ok!("中", '中');
        assert_ok!("東", '東');
        assert_ok!("δ", 'δ');
        assert_ok!("Δ", 'Δ');
    }

    #[test]
    fn test_constant_identifier() {
        use_parser!(constant_identifier, Input, String, ErrorKind);
        unimplemented!()
    }
}
