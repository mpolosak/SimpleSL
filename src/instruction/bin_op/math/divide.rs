use crate::{ExecError, variable::Variable};

pub fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
    match (dividend, divisor) {
        (_, Variable::Int(0)) => Err(ExecError::ZeroDivision),
        (Variable::Int(dividend), Variable::Int(divisor)) => {
            Ok(dividend.wrapping_div(divisor).into())
        }
        (Variable::Float(dividend), Variable::Float(divisor)) => Ok((dividend / divisor).into()),
        (dividend, divisor) => {
            panic!("Tried to calc {dividend} / {divisor}")
        }
    }
}
