use crate::binNumOp;
use crate::instruction::Instruction;
use crate::variable::{Typed, Variable};

binNumOp!(Multiply, "*");

impl Multiply {
    fn create_from_instructions(
        lhs: Instruction,
        rhs: Instruction,
    ) -> Result<Instruction, ExecError> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Self::exec(lhs, rhs)?.into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        })
    }

    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable, ExecError> {
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
            (lhs, rhs) => panic!("Tried to calc {lhs} * {rhs}"),
        }
    }
}
