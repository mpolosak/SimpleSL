use crate::binIntOp;
use crate::instruction::traits::CreateFromInstructions;
use crate::instruction::Instruction;
use crate::variable::Typed;
use crate::{variable::Variable, Result};

binIntOp!(BitwiseOr, "|");

impl CreateFromInstructions for BitwiseOr {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::bin_or(lhs, rhs).into()
            }
            (lhs, rhs) => Self::construct(lhs, rhs).into(),
        })
    }
}

impl BitwiseOr {
    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable> {
        Ok(Self::bin_or(lhs, rhs))
    }
    fn bin_or(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs | rhs).into(),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (value, Variable::Array(array)) | (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::bin_or(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
        }
    }
}
