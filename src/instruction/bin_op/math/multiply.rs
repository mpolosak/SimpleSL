use crate::{
    BinOperator,
    instruction::{Instruction, create_from_instructions_with_exec},
};

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    create_from_instructions_with_exec(lhs, rhs, BinOperator::Multiply, |lhs, rhs| lhs * rhs)
}
