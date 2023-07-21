use super::{local_variable::LocalVariableMap, Instruction};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    parse::Rule,
    variable::Variable,
};
use pest::iterators::Pair;

pub trait CreateInstruction {
    fn create_instruction(
        pair: Pair<Rule>,
        variables: &VariableMap,
        local_variables: &mut LocalVariableMap,
    ) -> Result<Instruction, Error>;
}

pub trait Exec {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error>;
}

pub trait Recreate {
    fn recreate(
        &self,
        local_variables: &mut LocalVariableMap,
        args: &VariableMap,
    ) -> Result<Instruction, Error>;
}
