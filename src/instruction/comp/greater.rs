use crate::binOpCBU;
use crate::instruction::macros::bin_num_op::ACCEPTED_TYPE;
use crate::instruction::{Exec, Instruction};
use crate::variable::{ReturnType, Type};
use crate::{interpreter::Interpreter, variable::Variable, Result};

binOpCBU!(Greater, ">");

impl Greater {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::greater(lhs, rhs).into())
            }
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable> {
        Ok(Self::greater(lhs, rhs))
    }

    fn greater(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs > rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs > rhs).into(),
            (lhs, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::greater(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::greater(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} > {rhs}"),
        }
    }
}

impl ReturnType for Greater {
    fn return_type(&self) -> Type {
        if matches!(
            (self.lhs.return_type(), self.rhs.return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            [Type::Int].into()
        } else {
            Type::Int
        }
    }
}
