use crate::instruction::{BinOperation, Instruction};
use crate::variable::Variable;
use crate::BinOperator;
use match_any::match_any;

pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
    match (lhs, rhs) {
        (Instruction::Variable(lhs), Instruction::Variable(rhs)) => exec(lhs, rhs).into(),
        (lhs, rhs) => BinOperation {
            lhs,
            rhs,
            op: BinOperator::Subtract,
        }
        .into(),
    }
}

pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
    match_any! { (lhs, rhs),
        (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
            => (lhs - rhs).into(),
        (lhs, rhs) => panic!("Tried to do {lhs} - {rhs}")
    }
}
