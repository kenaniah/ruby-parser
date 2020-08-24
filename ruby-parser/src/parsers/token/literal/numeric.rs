use crate::ast::{Literal, LiteralResult};
use crate::lexer::{CharResult, StringResult, TokenResult};
use crate::{Input, Token};
use nom::branch::alt;
use nom::character::complete::{anychar, char, one_of};
use nom::combinator::{map, opt, value, verify};
use nom::multi::many0;
use nom::sequence::{preceded, tuple};

/**
Provides support for lexing Ruby's Literal literal formats.

## How To Use

This function utilizes a set of [nom](https://docs.rs/nom/6.0.0-alpha1/nom/index.html) parser
combinators that follow the ISO Ruby spec for lexing numeric literals. This function returns a
tuple of the remaining input and a `Token::Integer` or `Token::Float` value when successful.

## Example

```
use ruby_parser::*;
use ruby_parser::ast::Literal;

let input = "12_345";
let (remaining, token) = parse(input.into()).unwrap();
assert_eq!("", *remaining);
assert_eq!(Token::Block(vec![Token::Literal(Literal::Integer(12345))]), token);

let input = "-12.34e+4 + 12";
let (remaining, token) = parse(input.into()).unwrap();
assert_eq!(" + 12", *remaining);
assert_eq!(Token::Block(vec![Token::Literal(Literal::Float(-123400.0))]), token);
```

## ISO Spec
8.7.6.2 - Numeric Literals

*signed_number* | *unsigned_number*
*/
pub(crate) fn numeric_literal(i: Input) -> TokenResult {
    // Ordered to match the largest production first
    let (i, num) = alt((signed_number, unsigned_number))(i)?;
    let token = Token::Literal(num);
    Ok((i, token))
}

/// ( `+` | `-` ) *unsigned_number*
pub(crate) fn signed_number(i: Input) -> LiteralResult {
    let (i, sign) = opt(one_of("+-"))(i)?;
    let (i, token) = unsigned_number(i)?;
    if sign == Some('-') {
        let token = match token {
            Literal::Integer(v) => Literal::Integer(v * -1),
            Literal::Float(v) => Literal::Float(v * -1f64),
            v => v,
        };
        return Ok((i, token));
    }
    Ok((i, token))
}

/// *float_literal* | *integer_literal*
pub(crate) fn unsigned_number(i: Input) -> LiteralResult {
    // Ordered to match the largest production first
    alt((float_literal, integer_literal))(i)
}

/// *binary_integer_literal* | *octal_integer_literal* | *hexadecimal_integer_literal* | *decimal_integer_literal*
pub(crate) fn integer_literal(i: Input) -> LiteralResult {
    // Ordered to match the largest production first
    alt((
        binary_integer_literal,
        octal_integer_literal,
        hexadecimal_integer_literal,
        decimal_integer_literal,
    ))(i)
}

/// *prefixed_decimal_integer_literal* | *unprefixed_decimal_integer_literal*
pub(crate) fn decimal_integer_literal(i: Input) -> LiteralResult {
    // Ordered to match the largest production first
    map(
        alt((
            prefixed_decimal_integer_literal,
            unprefixed_decimal_integer_literal,
        )),
        |s| Literal::Integer(isize::from_str_radix(&s, 10).unwrap()),
    )(i)
}

/// `0` | *decimal_digit_except_zero* ( `_`? *decimal_digit* )*
pub(crate) fn unprefixed_decimal_integer_literal(i: Input) -> StringResult {
    let (i, string) = alt((value(String::from("0"), char('0')), |i| {
        let (i, digit) = decimal_digit_except_zero(i)?;
        let (i, rest) = many0(preceded(opt(char('_')), decimal_digit))(i)?;
        Ok((i, concat(digit, rest)))
    }))(i)?;
    Ok((i, string))
}

/// `0` ( `d` | `D` ) *digit_decimal_part*
pub(crate) fn prefixed_decimal_integer_literal(i: Input) -> StringResult {
    let (i, digits) = preceded(char('0'), preceded(one_of("dD"), digit_decimal_part))(i)?;
    Ok((i, digits))
}

/// *decimal_digit* ( `_`? *decimal_digit* )*
pub(crate) fn digit_decimal_part(i: Input) -> StringResult {
    let (i, digit) = decimal_digit(i)?;
    let (i, rest) = many0(preceded(opt(char('_')), decimal_digit))(i)?;
    Ok((i, concat(digit, rest)))
}

/// `0` ( `b` | `B` ) *binary_digit* ( `_`? *binary_digit* )*
pub(crate) fn binary_integer_literal(i: Input) -> LiteralResult {
    let (i, digit) = preceded(char('0'), preceded(one_of("bB"), binary_digit))(i)?;
    let (i, rest) = many0(preceded(opt(char('_')), binary_digit))(i)?;
    Ok((
        i,
        Literal::Integer(isize::from_str_radix(&concat(digit, rest), 2).unwrap()),
    ))
}

/// `0` ( `_` | `o` | `O` )? *octal_digit* ( `_`? *octal_digit* )*
pub(crate) fn octal_integer_literal(i: Input) -> LiteralResult {
    let (i, digit) = preceded(char('0'), preceded(opt(one_of("_oO")), octal_digit))(i)?;
    let (i, rest) = many0(preceded(opt(char('_')), octal_digit))(i)?;
    Ok((
        i,
        Literal::Integer(isize::from_str_radix(&concat(digit, rest), 8).unwrap()),
    ))
}

/// `0` ( `x` | `X` ) *hexadecimal_digit* ( `_`? *hexadecimal_digit* )*
pub(crate) fn hexadecimal_integer_literal(i: Input) -> LiteralResult {
    let (i, digit) = preceded(char('0'), preceded(one_of("xX"), hexadecimal_digit))(i)?;
    let (i, rest) = many0(preceded(opt(char('_')), hexadecimal_digit))(i)?;
    Ok((
        i,
        Literal::Integer(isize::from_str_radix(&concat(digit, rest), 16).unwrap()),
    ))
}

/// *float_literal_with_exponent* | *float_literal_without_exponent*
pub(crate) fn float_literal(i: Input) -> LiteralResult {
    // Ordered to match the largest production first
    map(
        alt((float_literal_with_exponent, float_literal_without_exponent)),
        |s| Literal::Float(s.parse::<f64>().unwrap()),
    )(i)
}

/// *unprefixed_decimal_integer_literal* `.` *digit_decimal_part
pub(crate) fn float_literal_without_exponent(i: Input) -> StringResult {
    let (i, parts) = tuple((
        unprefixed_decimal_integer_literal,
        char('.'),
        digit_decimal_part,
    ))(i)?;
    let mut string = String::with_capacity(parts.0.len() + 1 + parts.2.len());
    string.push_str(&parts.0);
    string.push(parts.1);
    string.push_str(&parts.2);
    Ok((i, string))
}

/// *significand_part* *exponent_part*
pub(crate) fn float_literal_with_exponent(i: Input) -> StringResult {
    map(tuple((significand_part, exponent_part)), |t| {
        let mut string = String::with_capacity(t.0.len() + t.1.len());
        string.push_str(&t.0);
        string.push_str(&t.1);
        string
    })(i)
}

/// *float_literal_without_exponent* | *unprefixed_decimal_integer_literal*
pub(crate) fn significand_part(i: Input) -> StringResult {
    alt((
        float_literal_without_exponent,
        unprefixed_decimal_integer_literal,
    ))(i)
}

/// ( `e` | `E` ) ( `+` | `-` )?  *digit_decimal_part*
pub(crate) fn exponent_part(i: Input) -> StringResult {
    let (i, parts) = tuple((one_of("eE"), opt(one_of("+-")), digit_decimal_part))(i)?;
    let mut string = String::with_capacity(parts.2.len() + 2);
    string.push(parts.0);
    if let Some(sign) = parts.1 {
        string.push(sign);
    };
    string.push_str(&parts.2);
    Ok((i, string))
}

/// `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9`
pub(crate) fn decimal_digit_except_zero(i: Input) -> CharResult {
    one_of("123456789")(i)
}

/// `0` | `1`
pub(crate) fn binary_digit(i: Input) -> CharResult {
    verify(anychar, |c: &char| c.is_digit(2))(i)
}

/// `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7`
pub(crate) fn octal_digit(i: Input) -> CharResult {
    verify(anychar, |c: &char| c.is_digit(8))(i)
}

/// *decimal_digit* | `a` | `b` | `c` | `d` | `e` | `f` | `A` | `B` | `C` | `D` | `E` | `F`
pub(crate) fn hexadecimal_digit(i: Input) -> CharResult {
    verify(anychar, |c: &char| c.is_ascii_hexdigit())(i)
}

/// `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9`
pub(crate) fn decimal_digit(i: Input) -> CharResult {
    verify(anychar, |c: &char| c.is_ascii_digit())(i)
}

/// Constructs a string from characters
fn concat(chr: char, rest: Vec<char>) -> String {
    let mut string = String::with_capacity(chr.len_utf8() + rest.len());
    string.push(chr);
    string.push_str(&rest.into_iter().collect::<String>());
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_integer_literal() {
        use_parser!(binary_integer_literal);
        // Parse errors
        assert_err!("0");
        assert_err!("0b");
        assert_err!("2");
        assert_err!("0101");
        // Success cases
        assert_ok!("0b0", Literal::Integer(0));
        assert_ok!("0b0110", Literal::Integer(6));
        assert_ok!("0B0000_1111", Literal::Integer(15));
        assert_ok!("0b1111111101", Literal::Integer(1021));
        // Non-exhaustive cases
        assert_err!("0b1111 foobar");
        assert_err!("0b1251");
    }

    #[test]
    fn test_octal_integer_literal() {
        use_parser!(octal_integer_literal);
        // Parser errors
        assert_err!("0");
        assert_err!("0_");
        assert_err!("0b0");
        assert_err!("09");
        // Success cases
        assert_ok!("0_0", Literal::Integer(0));
        assert_ok!("00", Literal::Integer(0));
        assert_ok!("0o0", Literal::Integer(0));
        assert_ok!("0O0", Literal::Integer(0));
        assert_ok!("01234", Literal::Integer(668));
        assert_ok!("0_755", Literal::Integer(493));
        assert_ok!("0_00_10", Literal::Integer(8));
        // Non-exhaustive cases
        assert_err!(
            "0_1__0",
            Input::new_with_pos("__0", 3, 1, 4),
            crate::ErrorKind::Eof
        );
        assert_err!(
            "0755 foobar",
            Input::new_with_pos(" foobar", 4, 1, 5),
            crate::ErrorKind::Eof
        );
    }

    #[test]
    fn test_hexadecimal_integer_literal() {
        use_parser!(hexadecimal_integer_literal);
        // Parser errors
        assert_err!("0");
        assert_err!("0_");
        assert_err!("0x");
        assert_err!("0X");
        assert_err!("0AC");
        // Success cases
        assert_ok!("0x0", Literal::Integer(0));
        assert_ok!("0XF", Literal::Integer(15));
        assert_ok!("0xAB_CD_EF", Literal::Integer(11259375));
        assert_ok!("0x10", Literal::Integer(16));
        // Non-exhaustive cases
        assert_err!("0x14 ");
        assert_err!("0xAC foobar");
    }

    #[test]
    fn test_decimal_integer_literal() {
        use_parser!(decimal_integer_literal);
        // Parser errors
        assert_err!("foo");
        assert_err!("d20");
        // Success cases
        assert_ok!("0", Literal::Integer(0));
        assert_ok!("12034", Literal::Integer(12034));
        assert_ok!("0d0", Literal::Integer(0));
        assert_ok!("0D52", Literal::Integer(52));
        assert_ok!("5_923_032", Literal::Integer(5923032));
        assert_ok!("0d12_000", Literal::Integer(12000));
        // Non-exhaustive cases
        assert_err!("42_");
        assert_err!("0b0");
        assert_err!("5x0");
        assert_err!("0_12");
        assert_err!("12 ");
        assert_err!("1d_8");
    }

    #[test]
    fn test_signed_number() {
        use_parser!(signed_number);
        // Parser errors
        assert_err!("foo");
        assert_err!("d20");
        assert_err!("_10");
        assert_err!("0_FF");
        assert_err!("0xH");
        assert_err!("+1d0");
        assert_err!("-1d0");
        assert_err!("-1d");
        // Success cases
        assert_ok!("0", Literal::Integer(0));
        assert_ok!("29_0", Literal::Integer(290));
        assert_ok!("0b1111", Literal::Integer(15));
        assert_ok!("0xFF", Literal::Integer(255));
        // Positive
        assert_ok!("+0", Literal::Integer(0));
        assert_ok!("+29_0", Literal::Integer(290));
        assert_ok!("+0755", Literal::Integer(493));
        assert_ok!("+0b1111", Literal::Integer(15));
        assert_ok!("+0xFF", Literal::Integer(255));
        // Negative
        assert_ok!("-0", Literal::Integer(0));
        assert_ok!("-5", Literal::Integer(-5));
        assert_ok!("-1_2", Literal::Integer(-12));
        assert_ok!("-0b11", Literal::Integer(-3));
        assert_ok!("-0x0000_0000F", Literal::Integer(-15));
        assert_ok!("-0d20", Literal::Integer(-20));
        assert_ok!("-0755", Literal::Integer(-493));
        // Floats
        assert_ok!("0.0", Literal::Float(0.0));
        assert_ok!("+0.0", Literal::Float(0.0));
        assert_ok!("-0.0", Literal::Float(-0.0));
        assert_ok!("-12.345e-2", Literal::Float(-0.12345));
        assert_ok!("+12.4e0", Literal::Float(12.4));
        assert_ok!("0.312_24E7", Literal::Float(3122400.0));
    }

    #[test]
    fn test_exponent_part() {
        use_parser!(exponent_part);
        // Parser errors
        assert_err!("12e5");
        assert_err!("f+5");
        assert_err!("e");
        assert_err!("e+");
        assert_err!("e_5");
        assert_err!("e-_5");
        // Success cases
        assert_ok!("e3", "e3".to_owned());
        assert_ok!("e+0", "e+0".to_owned());
        assert_ok!("E-12", "E-12".to_owned());
        assert_ok!("e+0", "e+0".to_owned());
        assert_ok!("e0", "e0".to_owned());
        assert_ok!("e+12_000", "e+12000".to_owned());
        assert_ok!("E-4_5", "E-45".to_owned());
    }

    #[test]
    fn test_float_literal() {
        use_parser!(float_literal);
        // Parser errors
        assert_err!("0");
        assert_err!("0.");
        assert_err!("42");
        assert_err!("42e");
        assert_err!("-12.0");
        assert_err!("e");
        // Success cases
        assert_ok!("0.0", Literal::Float(0.));
        assert_ok!("12.0", Literal::Float(12.0));
        assert_ok!("12e0", Literal::Float(12.0));
        assert_ok!("12.34_005E-5", Literal::Float(0.0001234005));
        assert_ok!("1_23e+1_0", Literal::Float(1230000000000.0));
        assert_ok!("99.9e-0", Literal::Float(99.9));
        assert_ok!("1825_345e-12", Literal::Float(1825345e-12));
        // Non-exhaustive cases
        assert_err!("12.2492.");
        assert_err!("12.4+12");
    }
}
