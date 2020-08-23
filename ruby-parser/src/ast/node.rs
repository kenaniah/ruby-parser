use super::*;

pub enum Node {
    Literal,
    BinaryOp(BinaryOp)
}
