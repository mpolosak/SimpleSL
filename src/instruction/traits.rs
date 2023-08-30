mod bin_op;
mod can_be_used;
mod create_bin_op;
mod create_from_instructions;
mod prefix_op;
use std::fmt::Debug;

pub use self::{
    bin_op::BinOp, can_be_used::CanBeUsed, create_bin_op::CreateBinOp,
    create_from_instructions::CreateFromInstructions, prefix_op::PrefixOp,
};
use super::{local_variable::LocalVariables, Instruction};
use crate::{
    interpreter::Interpreter,
    parse::Rule,
    variable::{ReturnType, Variable},
    Result,
};
use pest::iterators::Pair;
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
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable>;
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
        Self::Other(Box::new(value))
    }
}
