use crate::{
    instruction::{create_from_instructions_with_exec, Instruction},
    variable::Variable,
    BinOperator,
};
use match_any::match_any;

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    create_from_instructions_with_exec(lhs, rhs, BinOperator::Subtract, exec)
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    match_any! { (lhs, rhs),
        (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
            => (lhs * rhs).into(),
        (lhs, rhs) => panic!("Tried to do {lhs} * {rhs}")
    }
}
