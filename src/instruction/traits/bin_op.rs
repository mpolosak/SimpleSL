use crate::instruction::Instruction;

pub trait BinOp {
    const SYMBOL: &'static str;
    fn get_lhs(&self) -> &Instruction;
    fn get_rhs(&self) -> &Instruction;
}
