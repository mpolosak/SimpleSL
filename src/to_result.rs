use crate::{ExecError, variable::Variable};

pub trait ToResult<T, E> {
    fn to_result(self) -> Result<T, E>;
}

impl<T: Into<Variable>> ToResult<Variable, ExecError> for T {
    fn to_result(self) -> Result<Variable, ExecError> {
        Ok(self.into())
    }
}

impl<T: Into<Variable>> ToResult<Variable, ExecError> for Result<T, ExecError> {
    fn to_result(self) -> Result<Variable, ExecError> {
        self.map(Into::into)
    }
}
