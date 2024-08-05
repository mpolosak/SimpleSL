use crate::{
    instruction::{
        bin_op::*,
        prefix_op::{BitwiseNot, Not, UnaryMinus},
    },
    variable::Variable,
    ExecError, Interpreter, ToResult,
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

#[duplicate_item(T;
    [BitwiseAnd]; [Xor]; [BitwiseOr]; [And]; [Or];
    [Divide]; [Modulo]; [Pow];
    [Greater]; [GreaterOrEqual]; [Lower]; [LowerOrEqual];
    [LShift]; [RShift]; 
)]
impl Exec for T {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Self::exec(lhs, rhs).to_result()
    }
}
