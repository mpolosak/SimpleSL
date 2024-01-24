use crate::instruction::Instruction;
use crate::variable::{Typed, Variable};
use crate::{binIntOp, Result};

binIntOp!(BitwiseAnd, "&");

impl BitwiseAnd {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::bin_and(lhs, rhs).into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        })
    }
}

impl BitwiseAnd {
    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable> {
        Ok(Self::bin_and(lhs, rhs))
    }
    fn bin_and(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs & rhs).into(),
            (var @ Variable::Array(_), _) | (_, var @ Variable::Array(_))
                if var.as_type() == Type::EmptyArray =>
            {
                var
            }
            (value, Variable::Array(array)) | (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::bin_and(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} & {rhs} which is imposible"),
        }
    }
}
