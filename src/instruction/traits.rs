use super::{local_variable::LocalVariableMap, Instruction};
use crate::{error::Error, interpreter::Interpreter, parse::Rule, variable::Variable};
use pest::iterators::Pair;

pub trait CreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &Interpreter,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error>;
}

pub trait Exec {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable, Error>;
}

pub trait Recreate {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        interpreter: &Interpreter,
    ) -> Result<Instruction, Error>;
}
