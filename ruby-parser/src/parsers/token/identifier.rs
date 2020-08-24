use crate::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, one_of};
use nom::combinator::{map, recognize, verify};
use nom::multi::many0;
use nom::sequence::tuple;

/// *local_variable_identifier* | *global_variable_identifier* | *class_variable_identifier* | *instance_variable_identifier* | *constant_identifier* | *method_only_identifier* | *assignment_like_method_identifier*
pub(crate) fn identifier(i: Input) -> TokenResult {
    // Reordered to use the longest production
    alt((
        method_only_identifier,
        assignment_like_method_identifier,
        local_variable_identifier,
        global_variable_identifier,
        class_variable_identifier,
        instance_variable_identifier,
        constant_identifier,
    ))(i)
}

/// ( *lowercase_character* | `_` ) *identifier_character**
pub(crate) fn local_variable_identifier(i: Input) -> TokenResult {
    map(
        recognize(tuple((
            alt((lowercase_character, char('_'))),
            many0(identifier_character),
        ))),
        |s| Token::LocalVariableIdentifier((*s).to_owned()),
    )(i)
}

/// `$` *identifier_start_character* *identifier_character**
pub(crate) fn global_variable_identifier(i: Input) -> TokenResult {
    map(
        recognize(tuple((
            char('$'),
            identifier_start_character,
            many0(identifier_character),
        ))),
        |s| Token::GlobalVariableIdentifier((*s).to_owned()),
    )(i)
}

/// `@@` *identifier_start_character* *identifier_character**
pub(crate) fn class_variable_identifier(i: Input) -> TokenResult {
    map(
        recognize(tuple((
            tag("@@"),
            identifier_start_character,
            many0(identifier_character),
        ))),
        |s| Token::ClassVariableIdentifier((*s).to_owned()),
    )(i)
}

/// `@` *identifier_start_character* *identifier_character**
pub(crate) fn instance_variable_identifier(i: Input) -> TokenResult {
    map(
        recognize(tuple((
            char('@'),
            identifier_start_character,
            many0(identifier_character),
        ))),
        |s| Token::InstanceVariableIdentifier((*s).to_owned()),
    )(i)
}

/// *uppercase_character* *identifier_character**
pub(crate) fn constant_identifier(i: Input) -> TokenResult {
    map(
        recognize(tuple((uppercase_character, many0(identifier_character)))),
        |s| Token::ConstantIdentifier((*s).to_owned()),
    )(i)
}

/// Returns any UTF-8 upper case character
fn uppercase_character(i: Input) -> CharResult {
    // Note: MRI also supports using unicode title case characters (in addition to uppercase)
    // rust currently does not have title case detection available in its internal unicode lib
    verify(anychar, |c: &char| c.is_uppercase())(i)
}

/// Returns any UTF-8 non-upper case character
fn lowercase_character(i: Input) -> CharResult {
    verify(anychar, |c: &char| {
        c.is_lowercase() || (!c.is_ascii() && !c.is_uppercase())
    })(i)
}

/// ( *constant_identifier* | *local_variable_identifier* ) ( `!` | `?` )
pub(crate) fn method_only_identifier(i: Input) -> TokenResult {
    map(
        recognize(tuple((
            alt((constant_identifier, local_variable_identifier)),
            one_of("!?"),
        ))),
        |s| Token::MethodIdentifier((*s).to_owned()),
    )(i)
}

/// ( *constant_identifier* | *local_variable_identifier* ) `=`
pub(crate) fn assignment_like_method_identifier(i: Input) -> TokenResult {
    map(
        recognize(tuple((
            alt((constant_identifier, local_variable_identifier)),
            char('='),
        ))),
        |s| Token::AssignmentMethodIdentifier((*s).to_owned()),
    )(i)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier_character() {
        use_parser!(identifier_character);
        // Parse errors
        assert_err!("");
        assert_err!(" ");
        assert_err!("=");
        assert_err!("!");
        assert_err!("\n");
        assert_err!("\0");
        // Success cases
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
        use_parser!(identifier_start_character);
        // Parse errors
        assert_err!("");
        assert_err!(" ");
        assert_err!("=");
        assert_err!("!");
        assert_err!("0");
        assert_err!("9");
        assert_err!("\n");
        assert_err!("\0");
        // Success cases
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
    fn test_identifier() {
        use crate::ast::IdentifierType::*;
        use_parser!(identifier);
        // Parse errors
        assert_err!("=");
        assert_err!("@");
        assert_err!("var\t");
        // Success cases
        assert_ok!("_", Token::ident("_", LocalVariable));
        assert_ok!("local", Token::ident("local", LocalVariable));
        assert_ok!("local_Var", Token::ident("local_Var", LocalVariable));
        assert_ok!("ClassName", Token::ident("ClassName", Constant));
        assert_ok!("FOO", Token::ident("FOO", Constant));
        assert_ok!("@_", Token::ident("@_", InstanceVariable));
        assert_ok!("@@prop", Token::ident("@@prop", ClassVariable));
        assert_ok!("$_foo", Token::ident("$_foo", GlobalVariable));
        assert_ok!("is_valid?", Token::ident("is_valid?", Method));
        assert_ok!("bang!", Token::ident("bang!", Method));
        assert_ok!("var=", Token::ident("var=", AssignmentMethod));
    }

    #[test]
    fn test_local_variable_identifier() {
        use_parser!(local_variable_identifier);
        // Parse errors
        assert_err!("A");
        assert_err!("02var");
        assert_err!("02var\0");
        assert_err!("var.");
        assert_err!("var=");
        assert_err!("var!");
        assert_err!("var?");
        // Success cases
        assert_ok!("_");
        assert_ok!("_foo");
        assert_ok!("_FOO");
        assert_ok!("_2392");
        assert_ok!("var");
        assert_ok!("var_2");
        assert_ok!("😀");
    }

    #[test]
    fn test_global_variable_identifier() {
        use_parser!(global_variable_identifier);
        // Parse errors
        assert_err!("var");
        assert_err!("$$var");
        assert_err!("$$");
        assert_err!("$?");
        assert_err!("$!");
        // Success cases
        assert_ok!("$_");
        assert_ok!("$var");
        assert_ok!("$VAR_");
        assert_ok!("$東");
    }

    #[test]
    fn test_class_variable_identifier() {
        use_parser!(class_variable_identifier);
        // Parse errors
        assert_err!("var");
        assert_err!("@@");
        assert_err!("@var");
        assert_err!("@@$foo");
        assert_err!("@@var?");
        // Success cases
        assert_ok!("@@var");
        assert_ok!("@@_");
        assert_ok!("@@FOO");
        assert_ok!("@@東");
    }

    #[test]
    fn test_instance_variable_identifier() {
        use_parser!(instance_variable_identifier);
        // Parse errors
        assert_err!("var");
        assert_err!("@");
        assert_err!("@@var");
        assert_err!("@$foo");
        assert_err!("@var?");
        // Success cases
        assert_ok!("@var");
        assert_ok!("@_");
        assert_ok!("@FOO");
        assert_ok!("@東");
    }

    #[test]
    fn test_constant_identifier() {
        use_parser!(constant_identifier);
        // Parse errors
        assert_err!("_PREFIXED");
        assert_err!("lowercase");
        assert_err!("$VAR");
        assert_err!("😀");
        assert_err!("C\0");
        // Success cases
        assert_ok!("FOOBAR");
        assert_ok!("CamelCase");
        assert_ok!("A");
        assert_ok!("Δ");
    }

    #[test]
    fn test_method_only_identifier() {
        use_parser!(method_only_identifier);
        // Parse errors
        assert_err!("var");
        assert_err!("var!?");
        // Success cases
        assert_ok!("var!");
        assert_ok!("var?");
        assert_ok!("Var?");
        assert_ok!("VAR!");
        assert_ok!("😀?");
    }

    #[test]
    fn test_assignment_like_method_identifier() {
        use_parser!(assignment_like_method_identifier);
        // Parse errors
        assert_err!("=");
        assert_err!("var");
        assert_err!("0=");
        assert_err!("var==");
        // Success cases
        assert_ok!("var=");
        assert_ok!("VAR=");
        assert_ok!("_=");
        assert_ok!("😀=");
    }
}
