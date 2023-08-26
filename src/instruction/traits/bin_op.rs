use crate::instruction::Instruction;

pub trait BinOp {
    const SYMBOL: &'static str;
    fn lhs(&self) -> &Instruction;
    fn rhs(&self) -> &Instruction;
    fn construct(lhs: Instruction, rhs: Instruction) -> Self;
}
