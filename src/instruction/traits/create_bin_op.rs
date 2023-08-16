use super::{BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::Instruction;
use crate::{variable::GetReturnType, Error, Result};

pub trait CreateBinOp {
    fn create_bin_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction>;
}

impl<T: BinOp + CanBeUsed + CreateFromInstructions> CreateBinOp for T {
    fn create_bin_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        let lhs_type = lhs.get_return_type();
        let rhs_type = rhs.get_return_type();
        if Self::can_be_used(&lhs_type, &rhs_type) {
            Self::create_from_instructions(lhs, rhs)
        } else {
            Err(Error::CannotDo2(lhs_type, Self::SYMBOL, rhs_type))
        }
    }
}
