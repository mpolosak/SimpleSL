use crate::instruction::{
    macros::{binIntOp, bin_int_op::ACCEPTED_TYPE},
    Instruction,
};
use crate::variable::Variable;
use crate::ExecError;

binIntOp!(Modulo, "%");

impl Modulo {
    pub fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::exec(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(ExecError::ZeroModulo),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(ExecError::ZeroModulo),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend % divisor).into()),
            (Variable::Array(array), divisor @ Variable::Int(_)) => array
                .iter()
                .cloned()
                .map(|dividend| Self::exec(dividend, divisor.clone()))
                .collect::<Result<Variable, ExecError>>(),
            (dividend @ Variable::Int(_), Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|divisor| Self::exec(dividend.clone(), divisor))
                .collect::<Result<Variable, ExecError>>(),
            (dividend, divisor) => panic!("Tried to calc {dividend} % {divisor}"),
        }
    }
}
