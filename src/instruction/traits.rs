mod recreate;
pub use self::recreate::Recreate;

use super::{local_variable::LocalVariables, Instruction};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, ExecError,
};
use pest::iterators::Pair;
use std::{fmt::Debug, rc::Rc};
pub trait CreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction, Error>;
}

pub trait MutCreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error>;
}

pub trait Exec {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult;
}

pub trait BaseInstruction: Exec + Recreate + ReturnType + Debug {}

impl<T: BaseInstruction + 'static> From<T> for Instruction {
    fn from(value: T) -> Self {
        Self::Other(Rc::new(value))
    }
}

pub trait CanBeUsed {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool;
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

pub trait ToResult<T, E> {
    fn to_result(self) -> Result<T, E>;
}

impl<T, E> ToResult<T, E> for T {
    fn to_result(self) -> Result<T, E> {
        Ok(self)
    }
}

impl<T, E0, E1: From<E0>> ToResult<T, E1> for Result<T, E0> {
    fn to_result(self) -> Result<T, E1> {
        self.map_err(E1::from)
    }
}
