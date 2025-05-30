use crate::{
    BinOperator, ExecError,
    instruction::{BinOperation, Instruction},
    variable::Variable,
};

pub fn create_from_instructions(
    dividend: Instruction,
    divisor: Instruction,
) -> Result<Instruction, ExecError> {
    match (dividend, divisor) {
        (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
            Ok(exec(dividend, divisor)?.into())
        }
        (_, Instruction::Variable(Variable::Int(0))) => Err(ExecError::ZeroModulo),
        (lhs, rhs) => Ok(BinOperation {
            lhs,
            rhs,
            op: BinOperator::Modulo,
        }
        .into()),
    }
}

pub fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
    match (dividend, divisor) {
        (_, Variable::Int(0)) => Err(ExecError::ZeroModulo),
        (Variable::Int(dividend), Variable::Int(divisor)) => {
            Ok((dividend.wrapping_rem(divisor)).into())
        }
        (dividend, divisor) => {
            panic!("Tried to calc {dividend} % {divisor}")
        }
    }
}
