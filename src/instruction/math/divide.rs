use crate::instruction::macros::ACCEPTED_NUM_TYPE as ACCEPTED_TYPE;
use crate::instruction::{macros::binOpCBU, Instruction};
use crate::variable::Variable;
use crate::ExecError;

binOpCBU!(Divide, "/");

impl Divide {
    pub fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::exec(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(ExecError::ZeroDivision),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    pub fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(ExecError::ZeroDivision),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend / divisor).into()),
            (Variable::Float(dividend), Variable::Float(divisor)) => {
                Ok((dividend / divisor).into())
            }
            (Variable::Array(array), divisor @ (Variable::Int(_) | Variable::Float(_))) => array
                .iter()
                .cloned()
                .map(|dividend| Self::exec(dividend, divisor.clone()))
                .collect::<Result<Variable, ExecError>>(),
            (dividend @ (Variable::Int(_) | Variable::Float(_)), Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|divisor| Self::exec(dividend.clone(), divisor))
                .collect::<Result<Variable, ExecError>>(),
            (dividend, divisor) => panic!("Tried to calc {dividend} / {divisor}"),
        }
    }
}
