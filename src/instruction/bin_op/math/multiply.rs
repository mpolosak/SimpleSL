use crate::{
    BinOperator,
    instruction::{Instruction, create_from_instructions_with_exec},
    variable::Variable,
};

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    create_from_instructions_with_exec(lhs, rhs, BinOperator::Multiply, exec)
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    match (lhs, rhs) {
        (Variable::Int(lhs), Variable::Int(rhs)) => lhs.wrapping_mul(rhs).into(),
        (Variable::Float(lhs), Variable::Float(rhs)) => (lhs * rhs).into(),
        (lhs, rhs) => panic!("Tried to do {lhs} * {rhs}"),
    }
}
