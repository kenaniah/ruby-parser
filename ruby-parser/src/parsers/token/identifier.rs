use crate::lexer::*;
use crate::parsers::token::keyword::keyword;

/// *local_variable_identifier* | *global_variable_identifier* | *class_variable_identifier* | *instance_variable_identifier* | *constant_identifier* | *method_only_identifier* | *assignment_like_method_identifier*
pub(crate) fn identifier(i: Input) -> NodeResult {
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
pub(crate) fn local_variable_identifier(i: Input) -> NodeResult {
    use crate::nom::InputLength;
    map(
        verify(
            recognize(tuple((
                alt((lowercase_character, char('_'))),
                many0(identifier_character),
            ))),
            // Ensure that we didn't match a known keyword
            |i| match keyword(i.clone()) {
                Ok((i, _)) => i.input_len() != 0,
                _ => true,
            },
        ),
        |s| Node::ident(*s, IdentifierKind::LocalVariable),
    )(i)
}

/// `$` *identifier_start_character* *identifier_character**
pub(crate) fn global_variable_identifier(i: Input) -> NodeResult {
    map(
        recognize(tuple((
            char('$'),
            identifier_start_character,
            many0(identifier_character),
        ))),
        |s| Node::ident(*s, IdentifierKind::GlobalVariable),
    )(i)
}

/// `@@` *identifier_start_character* *identifier_character**
pub(crate) fn class_variable_identifier(i: Input) -> NodeResult {
    map(
        recognize(tuple((
            tag("@@"),
            identifier_start_character,
            many0(identifier_character),
        ))),
        |s| Node::ident(*s, IdentifierKind::ClassVariable),
    )(i)
}

/// `@` *identifier_start_character* *identifier_character**
pub(crate) fn instance_variable_identifier(i: Input) -> NodeResult {
    map(
        recognize(tuple((
            char('@'),
            identifier_start_character,
            many0(identifier_character),
        ))),
        |s| Node::ident(*s, IdentifierKind::InstanceVariable),
    )(i)
}

/// *uppercase_character* *identifier_character**
pub(crate) fn constant_identifier(i: Input) -> NodeResult {
    use crate::nom::InputLength;
    map(
        verify(
            recognize(tuple((uppercase_character, many0(identifier_character)))),
            // Ensure that we didn't match a known keyword
            |i| match keyword(i.clone()) {
                Ok((i, _)) => i.input_len() != 0,
                _ => true,
            },
        ),
        |s| Node::ident(*s, IdentifierKind::Constant),
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
pub(crate) fn method_only_identifier(i: Input) -> NodeResult {
    map(
        recognize(tuple((
            alt((constant_identifier, local_variable_identifier)),
            one_of("!?"),
        ))),
        |s| Node::ident(*s, IdentifierKind::Method),
    )(i)
}

/// ( *constant_identifier* | *local_variable_identifier* ) `=`
pub(crate) fn assignment_like_method_identifier(i: Input) -> NodeResult {
    map(
        recognize(tuple((
            alt((constant_identifier, local_variable_identifier)),
            char('='),
        ))),
        |s| Node::ident(*s, IdentifierKind::AssignmentMethod),
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
        use crate::ast::IdentifierKind::*;
        use_parser!(identifier);
        // Parse errors
        assert_err!("=");
        assert_err!("@");
        assert_err!("var\t");
        assert_err!("not");
        assert_err!("if");
        assert_err!("nil");
        assert_err!("true");
        assert_err!("BEGIN");
        assert_err!("END");
        // Success cases
        assert_ok!("_", Node::ident("_", LocalVariable));
        assert_ok!("local", Node::ident("local", LocalVariable));
        assert_ok!("local_Var", Node::ident("local_Var", LocalVariable));
        assert_ok!("ClassName", Node::ident("ClassName", Constant));
        assert_ok!("FOO", Node::ident("FOO", Constant));
        assert_ok!("@_", Node::ident("@_", InstanceVariable));
        assert_ok!("@@prop", Node::ident("@@prop", ClassVariable));
        assert_ok!("$_foo", Node::ident("$_foo", GlobalVariable));
        assert_ok!("is_valid?", Node::ident("is_valid?", Method));
        assert_ok!("bang!", Node::ident("bang!", Method));
        assert_ok!("var=", Node::ident("var=", AssignmentMethod));
        assert_ok!("truely", Node::ident("truely", LocalVariable));
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
