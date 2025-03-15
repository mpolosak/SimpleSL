use crate::BinOperator;
use crate::instruction::{Instruction, create_from_instructions_with_exec};
use crate::variable::Variable;

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    create_from_instructions_with_exec(lhs, rhs, BinOperator::Subtract, exec)
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    match (lhs, rhs) {
        (Variable::Int(lhs), Variable::Int(rhs)) => lhs.wrapping_sub(rhs).into(),
        (Variable::Float(lhs), Variable::Float(rhs)) => (lhs - rhs).into(),
        (lhs, rhs) => panic!("Tried to do {lhs} - {rhs}"),
    }
}
