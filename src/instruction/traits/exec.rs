use crate::{
    instruction::prefix_op::{BitwiseNot, Not, UnaryMinus},
    variable::Variable,
    ExecError, Interpreter,
};
use duplicate::duplicate_item;

pub trait Exec {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult;
}

pub type ExecResult = Result<Variable, ExecStop>;
pub enum ExecStop {
    Return(Variable),
    Error(ExecError),
}

impl From<ExecError> for ExecStop {
    fn from(value: ExecError) -> Self {
        Self::Error(value)
    }
}

#[duplicate_item(T; [BitwiseNot]; [Not]; [UnaryMinus])]
impl Exec for T {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let result = self.instruction.exec(interpreter)?;
        Ok(Self::calc(result))
    }
}
