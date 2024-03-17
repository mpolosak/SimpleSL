use crate::instruction::{macros::binNumOp, Instruction};
use crate::variable::{Typed, Variable};

binNumOp!(Subtract, "-");

impl Subtract {
    pub fn create_from_instructions(minuend: Instruction, subtrahend: Instruction) -> Instruction {
        match (minuend, subtrahend) {
            (Instruction::Variable(minuend), Instruction::Variable(rhs)) => {
                Self::exec(minuend, rhs).into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }

    fn exec(minuend: Variable, subtrahend: Variable) -> Variable {
        match (minuend, subtrahend) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs - rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs - rhs).into(),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (minuend, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|subtrahend| Self::exec(minuend.clone(), subtrahend))
                .collect(),
            (Variable::Array(array), subtrahend) => array
                .iter()
                .cloned()
                .map(|minuend| Self::exec(minuend, subtrahend.clone()))
                .collect(),
            (minuend, subtrahend) => {
                panic!("Tried to calc {minuend} - {subtrahend}")
            }
        }
    }
}
