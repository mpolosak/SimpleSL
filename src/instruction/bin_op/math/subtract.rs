use crate::BinOperator;
use crate::instruction::{Instruction, create_from_instructions_with_exec};

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    create_from_instructions_with_exec(lhs, rhs, BinOperator::Subtract, |lhs, rhs| lhs - rhs)
}
