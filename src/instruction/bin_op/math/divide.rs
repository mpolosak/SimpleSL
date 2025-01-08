use crate::{
    instruction::{BinOperation, Instruction},
    variable::Variable,
    BinOperator, ExecError,
};

pub fn create_from_instructions(
    dividend: Instruction,
    divisor: Instruction,
) -> Result<Instruction, ExecError> {
    match (dividend, divisor) {
        (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
            Ok(exec(dividend, divisor)?.into())
        }
        (_, Instruction::Variable(Variable::Int(0))) => Err(ExecError::ZeroDivision),
        (lhs, rhs) => Ok(BinOperation {
            lhs,
            rhs,
            op: BinOperator::Divide,
        }
        .into()),
    }
}

pub fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
    match (dividend, divisor) {
        (_, Variable::Int(0)) => Err(ExecError::ZeroDivision),
        (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend / divisor).into()),
        (Variable::Float(dividend), Variable::Float(divisor)) => Ok((dividend / divisor).into()),
        (dividend, divisor) => {
            panic!("Tried to calc {dividend} / {divisor}")
        }
    }
}
