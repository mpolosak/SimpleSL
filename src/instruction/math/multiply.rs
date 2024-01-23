use crate::binNumOp;
use crate::instruction::traits::CreateFromInstructions;
use crate::instruction::Instruction;
use crate::variable::Typed;
use crate::{variable::Variable, Result};

binNumOp!(Multiply, "*");

impl CreateFromInstructions for Multiply {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::exec(lhs, rhs).map(Instruction::from)
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl Multiply {
    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable> {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((lhs * rhs).into()),
            (Variable::Float(lhs), Variable::Float(rhs)) => Ok((lhs * rhs).into()),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                Ok(array)
            }
            (value, Variable::Array(array)) | (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::exec(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to calc {lhs} {} {rhs}", Self::SYMBOL),
        }
    }
}
