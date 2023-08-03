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
