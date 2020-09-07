use super::double::double_escape_sequence;
use super::double::interpolated_character_sequence;
use crate::lexer::*;

/// `%q` *non_expanded_delimited_string*
pub(crate) fn quoted_non_expanded_literal_string(i: Input) -> StringResult {
    preceded(tag("%q"), non_expanded_delimited_string)(i)
}

/// `%` `Q`? *expanded_delimited_string*
pub(crate) fn quoted_expanded_literal_string(i: Input) -> InterpolatableResult {
    preceded(alt((tag("%Q"), tag("%"))), expanded_delimited_string)(i)
}

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn non_expanded_delimited_string(i: Input) -> StringResult {
    map(
        wrap_quote_delimiter(delimited(
            literal_beginning_delimiter,
            many0(non_expanded_literal_string),
            literal_ending_delimiter,
        )),
        |vec| {
            let mut s = String::new();
            for str in vec {
                s.push_str(&str);
            }
            s
        },
    )(i)
}

/// Manages the state of the input's quote delimiter when nested
fn wrap_quote_delimiter<'a, O1, E, F>(
    mut func: F,
) -> impl FnMut(Input<'a>) -> nom::IResult<Input<'a>, O1, E>
where
    F: nom::Parser<Input<'a>, O1, E>,
{
    move |mut i: Input<'a>| {
        let delim = i.metadata.quote_delimiter;
        i.metadata.quote_delimiter = None;
        let res = func.parse(i);
        match res {
            Ok((mut i, o1)) => {
                i.metadata.quote_delimiter = delim;
                Ok((i, o1))
            }
            error @ _ => error,
        }
    }
}

/// *literal_beginning_delimiter* *non_expanded_literal_string** *literal_ending_delimiter*
fn _non_expanded_delimited_string(i: Input) -> StringResult {
    map(
        tuple((
            literal_beginning_delimiter,
            many0(non_expanded_literal_string),
            literal_ending_delimiter,
        )),
        |t| {
            let mut s = String::new();
            s.push(t.0);
            for str in t.1 {
                s.push_str(&str);
            }
            s.push(t.2);
            s
        },
    )(i)
}

/// *literal_beginning_delimiter* *expanded_literal_string** *literal_ending_delimiter*
pub(crate) fn expanded_delimited_string(i: Input) -> InterpolatableResult {
    map(
        wrap_quote_delimiter(delimited(
            literal_beginning_delimiter,
            many0(expanded_literal_string),
            literal_ending_delimiter,
        )),
        |vecs| Interpolatable::from(vecs.into_iter().flatten().collect::<Vec<Segment>>()),
    )(i)
}

/// *literal_beginning_delimiter* *expanded_literal_string** *literal_ending_delimiter*
fn _expanded_delimited_string(i: Input) -> SegmentVecResult {
    map(
        tuple((
            literal_beginning_delimiter,
            many0(expanded_literal_string),
            literal_ending_delimiter,
        )),
        |mut t| {
            t.1.insert(0, vec![Segment::Char(t.0)]);
            t.1.push(vec![Segment::Char(t.2)]);
            t.1.into_iter().flatten().collect()
        },
    )(i)
}

/// *non_expanded_literal_character* | *non_expanded_delimited_string*
pub(crate) fn non_expanded_literal_string(i: Input) -> StringResult {
    alt((
        non_expanded_literal_character,
        _non_expanded_delimited_string,
    ))(i)
}

/// *expanded_literal_character* | *expanded_delimited_string*
pub(crate) fn expanded_literal_string(i: Input) -> SegmentVecResult {
    alt((
        map(expanded_literal_character, |s| vec![s]),
        _expanded_delimited_string,
    ))(i)
}

/// *non_escaped_literal_character* | *non_expanded_literal_escape_sequence*
pub(crate) fn non_expanded_literal_character(i: Input) -> StringResult {
    alt((
        non_escaped_literal_character,
        non_expanded_literal_escape_sequence,
    ))(i)
}

/// *non_escaped_literal_character* **but not** `#` | `#` **not** ( `$` | `@` | `{` ) | *double_escape_sequence* | *interpolated_character_sequence*
pub(crate) fn expanded_literal_character(i: Input) -> SegmentResult {
    alt((
        map(
            preceded(peek(not(char('#'))), non_escaped_literal_character),
            |s| Segment::String(s),
        ),
        map(double_escape_sequence, |s| Segment::String(s)),
        map(interpolated_character_sequence, |e| Segment::expr(e)),
        map(terminated(char('#'), peek(none_of("$@{"))), |c| {
            Segment::Char(c)
        }),
    ))(i)
}

/// *source_character* **but not** *quoted_literal_escape_character*
pub(crate) fn non_escaped_literal_character(i: Input) -> StringResult {
    preceded(
        peek(not(quoted_literal_escape_character)),
        map(anychar, |c| c.to_string()),
    )(i)
}

/// *non_expanded_literal_escape_character_sequence* | *non_escaped_non_expanded_literal_character_sequence*
pub(crate) fn non_expanded_literal_escape_sequence(i: Input) -> StringResult {
    alt((
        non_expanded_literal_escape_character_sequence,
        non_escaped_non_expanded_literal_character_sequence,
    ))(i)
}

/// `\` *non_expanded_literal_escaped_character*
pub(crate) fn non_expanded_literal_escape_character_sequence(i: Input) -> StringResult {
    preceded(char('\\'), non_expanded_literal_escaped_character)(i)
}

/// *literal_beginning_delimiter* | *literal_ending_delimiter* | `\`
pub(crate) fn non_expanded_literal_escaped_character(i: Input) -> StringResult {
    map(
        alt((
            literal_beginning_delimiter,
            literal_ending_delimiter,
            char('\\'),
        )),
        |c| c.to_string(),
    )(i)
}

/// *non_expanded_literal_escaped_character*
pub(crate) fn quoted_literal_escape_character(i: Input) -> StringResult {
    non_expanded_literal_escaped_character(i)
}

/// `\` *non_escaped_non_expanded_literal_character*
pub(crate) fn non_escaped_non_expanded_literal_character_sequence(i: Input) -> StringResult {
    map(
        tuple((char('\\'), non_escaped_non_expanded_literal_character)),
        |t| {
            let mut s = String::new();
            s.push(t.0);
            s.push_str(&t.1);
            s
        },
    )(i)
}

/// *source_character* **but not** *non_expanded_literal_escaped_character*
pub(crate) fn non_escaped_non_expanded_literal_character(i: Input) -> StringResult {
    preceded(
        peek(not(non_expanded_literal_escaped_character)),
        map(anychar, |c| c.to_string()),
    )(i)
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_beginning_delimiter(i: Input) -> CharResult {
    let (mut i, c) = match start_delimiter(&i.metadata) {
        Some(c) => char(c)(i)?,
        None => verify(anychar, |c: &char| c.is_ascii_punctuation())(i)?,
    };
    i.metadata.quote_delimiter = Some(c);
    Ok((i, c))
}

/// *source_character* **but not** *alpha_numeric_character*
pub(crate) fn literal_ending_delimiter(i: Input) -> CharResult {
    match end_delimiter(&i.metadata) {
        Some(c) => char(c)(i),
        None => unimplemented!(),
    }
}

fn start_delimiter(meta: &Metadata) -> Option<char> {
    meta.quote_delimiter
}

fn end_delimiter(meta: &Metadata) -> Option<char> {
    match meta.quote_delimiter {
        Some('{') => Some('}'),
        Some('(') => Some(')'),
        Some('[') => Some(']'),
        Some('<') => Some('>'),
        _ => meta.quote_delimiter,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_string {
        ($a:expr, $b:expr) => {
            assert_ok!($a, Interpolatable::String($b.to_owned()))
        };
    }
    macro_rules! assert_interpolated {
        ($a:expr, $b:expr) => {
            assert_ok!($a, Interpolatable::Interpolated($b))
        };
    }

    #[test]
    fn test_quoted_expanded_literal_string() {
        use_parser!(quoted_expanded_literal_string);
        // Parse errors
        assert_err!("%(");
        assert_err!("%Q(");
        assert_err!("%(()");
        assert_err!("%(#{foo)");
        // Success cases
        assert_string!("%::", "");
        assert_string!("%Q<foo \\<bar>", "foo <bar");
        assert_string!("%Q<#12>", "#12");
        assert_interpolated!(
            "%[foo#@hi [bar] [#{%Q((hello))}]]",
            vec![
                Node::Segment(Segment::String("foo".to_owned())),
                Node::ident("@hi", IdentifierKind::InstanceVariable),
                Node::Segment(Segment::String(" [bar] [".to_owned())),
                Node::Block(vec![Node::literal_string("(hello)")]),
                Node::Segment(Segment::String("]".to_owned())),
            ]
        );
    }

    #[test]
    fn test_quoted_non_expanded_literal_string() {
        use_parser!(quoted_non_expanded_literal_string);
        // Parse errors
        assert_err!("%q(");
        assert_err!("%q((");
        assert_err!("%q(");
        assert_err!("%q(>");
        assert_err!("%q:");
        assert_err!("%q{foo");
        assert_err!("%q(foo)bar");
        assert_err!("%q[[abc] [def]");
        assert_err!("%q[[abc] [def])");
        assert_err!("%q((abc\\))");
        // Success cases
        assert_ok!("%q()", "");
        assert_ok!("%q((abc))", "(abc)");
        assert_ok!("%q((abc\\)))", "(abc))");
        assert_ok!("%q(foobar)", "foobar");
        assert_ok!("%q<\0>", "\0");
        assert_ok!("%q:foo\nbar:", "foo\nbar");
        assert_ok!("%q:foo\\n\\:bar\\\\:", "foo\\n:bar\\");
        assert_ok!("%q%Smiley 😂 here!%", "Smiley 😂 here!");
        assert_ok!("%q[[abc] [def]]", "[abc] [def]");
        assert_ok!("%q[\\[abc\\)def(]", "[abc\\)def(");
        assert_ok!("%q{{{\\{}}}", "{{{}}");
    }

    #[test]
    fn test_literal_beginning_delimiter() {
        use_parser!(literal_beginning_delimiter);
        // Parse errors
        assert_err!("");
        assert_err!("a");
        assert_err!(" ");
        assert_err!("5");
        assert_err!("東"); // U+6771: 'CJK Unified Ideograph-6771' "East"
        assert_err!("\n");
        // Success cases
        assert_ok!("<", '<');
        assert_ok!("(", '(');
        assert_ok!("}", '}');
        assert_ok!(":", ':');
        assert_ok!("_", '_');
        assert_ok!("\\", '\\');
    }
}
