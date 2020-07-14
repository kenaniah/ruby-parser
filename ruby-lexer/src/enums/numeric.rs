/// Internal enum used by the numeric_literal parser
#[derive(Debug, PartialEq)]
pub(crate) enum Numeric {
    Integer(isize),
    Float(f64),
}
