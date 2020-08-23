use super::*;

pub enum Node {
    Literal(Literal),
    Interpolated(Interpolated),
    BinaryOp(BinaryOp)
}
