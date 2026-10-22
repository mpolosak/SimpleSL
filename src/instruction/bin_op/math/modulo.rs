use crate::{ExecError, variable::Variable};

pub fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
    match (dividend, divisor) {
        (_, Variable::Int(0)) => Err(ExecError::ZeroModulo),
        (dividend, divisor) => Ok(dividend % divisor),
    }
}

#[cfg(test)]
mod tests {
    use super::exec;
    use crate::{ExecError, variable::Variable};

    #[test]
    fn calculates_integer_remainder_using_variable_operator() {
        assert_eq!(
            exec(Variable::Int(17), Variable::Int(5)),
            Ok(Variable::Int(2))
        );
    }

    #[test]
    fn returns_error_for_integer_modulo_by_zero() {
        assert_eq!(
            exec(Variable::Int(17), Variable::Int(0)),
            Err(ExecError::ZeroModulo)
        );
    }
}
