use super::{local_variable::LocalVariables, Instruction};
use crate::{error::Error, interpreter::Interpreter, parse::Rule, variable::Variable};
use pest::iterators::Pair;

pub trait CreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &Interpreter,
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
