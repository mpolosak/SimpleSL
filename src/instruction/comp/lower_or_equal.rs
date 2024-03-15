use super::return_type;
use crate::instruction::macros::{binOpCBU, bin_num_op::ACCEPTED_TYPE};
use crate::instruction::{Exec, Instruction};
use crate::variable::{ReturnType, Type};
use crate::{interpreter::Interpreter, variable::Variable};
use match_any::match_any;

binOpCBU!(LowerOrEqual, "<=");

impl LowerOrEqual {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }

    fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match_any! { (lhs, rhs),
            (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
                => (lhs <= rhs).into(),
            (lhs, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::exec(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::exec(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} <= {rhs}")
        }
    }
}

impl ReturnType for LowerOrEqual {
    fn return_type(&self) -> Type {
        let lhs = self.lhs.return_type();
        let rhs = self.rhs.return_type();
        return_type(lhs, rhs)
    }
}
