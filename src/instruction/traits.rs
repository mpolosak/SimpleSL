use super::{local_variable::LocalVariables, Instruction};
use crate::{interpreter::Interpreter, parse::Rule, variable::Variable, Error};
use pest::iterators::Pair;

pub trait CreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction, Error>;
}

pub trait Exec {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error>;
}

pub trait Recreate {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error>;
}
