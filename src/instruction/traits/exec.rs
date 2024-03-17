use crate::{
    instruction::{
        bitwise::{BitwiseAnd, BitwiseOr, Xor},
        logic::{And, Or},
        math::{Add, Divide, Modulo, Multiply, Pow, Subtract},
        ord::{Greater, GreaterOrEqual, Lower, LowerOrEqual},
        prefix_op::{BitwiseNot, Not, UnaryMinus},
        shift::{LShift, RShift},
    },
    variable::Variable,
    ExecError, Interpreter,
};
use duplicate::duplicate_item;

use super::ToResult;

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
    [BitwiseAnd]; [BitwiseOr]; [Xor]; [And]; [Or];
    [Add]; [Subtract]; [Multiply]; [Divide]; [Modulo]; [Pow];
    [Greater]; [GreaterOrEqual]; [Lower]; [LowerOrEqual];
    [LShift]; [RShift]
)]
impl Exec for T {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Self::exec(lhs, rhs).to_result()
    }
}
