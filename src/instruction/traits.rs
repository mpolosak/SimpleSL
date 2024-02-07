use super::{local_variable::LocalVariables, Instruction};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Type, Variable},
    Error, Result,
};
use pest::iterators::Pair;
use std::{fmt::Debug, rc::Rc, result};

pub trait CreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &LocalVariables,
    ) -> Result<Instruction>;
}

pub trait MutCreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction>;
}

pub trait Exec {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult;
}

pub trait Recreate {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction>;
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

pub type ExecResult = result::Result<Variable, ExecStop>;
pub enum ExecStop {
    Return(Variable),
    Error(Error),
}

impl From<Error> for ExecStop {
    fn from(value: Error) -> Self {
        Self::Error(value)
    }
}
