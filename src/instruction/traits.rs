use super::{
    block::Block,
    control_flow::{IfElse, Match, SetIfElse},
    destruct_tuple::DestructTuple,
    function::FunctionDeclaration,
    import::Import,
    local_variable::LocalVariables,
    reduce::Reduce,
    set::Set,
    type_filter::TypeFilter,
    FunctionCall, Instruction,
};
use crate::{
    variable::{ReturnType, Variable},
    ExecError, Interpreter,
};
use duplicate::duplicate_item;
use std::{fmt::Debug, sync::Arc};

pub trait BaseInstruction: Exec + Recreate + ReturnType + Debug + Sync + Send {}

#[duplicate_item(T; [Reduce]; [TypeFilter]; [Block]; [IfElse]; [Match]; [SetIfElse];
    [DestructTuple]; [FunctionCall]; [FunctionDeclaration]; [Import]; [Set];
)]
impl BaseInstruction for T {}

impl<T: BaseInstruction + 'static> From<T> for Instruction {
    fn from(value: T) -> Self {
        Self::Other(Arc::new(value))
    }
}

pub trait Recreate {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError>;
}

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
