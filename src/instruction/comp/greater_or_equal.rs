use crate::binOpCBU;
use crate::instruction::macros::bin_num_op::ACCEPTED_TYPE;
use crate::instruction::{Exec, Instruction};
use crate::variable::{ReturnType, Type};
use crate::{interpreter::Interpreter, variable::Variable};

binOpCBU!(GreaterOrEqual, ">=");

impl GreaterOrEqual {
    fn create_from_instructions(
        lhs: Instruction,
        rhs: Instruction,
    ) -> Result<Instruction, ExecError> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::greater_or_equal(lhs, rhs).into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        })
    }
}

impl GreaterOrEqual {
    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable, ExecError> {
        Ok(Self::greater_or_equal(lhs, rhs))
    }
    fn greater_or_equal(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs >= rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs >= rhs).into(),
            (lhs, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::greater_or_equal(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::greater_or_equal(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} >= {rhs}"),
        }
    }
}

impl ReturnType for GreaterOrEqual {
    fn return_type(&self) -> Type {
        if matches!(
            (self.lhs.return_type(), self.rhs.return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            return [Type::Int].into();
        }
        Type::Int
    }
}
