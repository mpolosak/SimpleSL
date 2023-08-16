mod bin_op;
mod can_be_used;
mod create_from_instructions;
pub use self::{
    bin_op::BinOp, can_be_used::CanBeUsed, create_from_instructions::CreateFromInstructions,
};
use super::{local_variable::LocalVariables, Instruction};
use crate::{interpreter::Interpreter, parse::Rule, variable::Variable, Result};
use pest::iterators::Pair;
pub trait CreateInstruction {
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
