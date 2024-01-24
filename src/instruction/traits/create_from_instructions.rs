use crate::instruction::Instruction;
use crate::Result;

pub trait CreateFromInstructions {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction>;
}
