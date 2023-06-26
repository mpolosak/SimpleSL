use super::{local_variable::LocalVariableMap, Instruction};
use crate::{
    error::Error,
    interpreter::{Interpreter, VariableMap},
    variable::Variable,
};

pub trait Exec {
    fn exec(
        &self,
        interpreter: &mut Interpreter,
        local_variables: &mut VariableMap,
    ) -> Result<Variable, Error>;
}

pub trait Recreate {
    fn recreate(self, local_variables: &mut LocalVariableMap, args: &VariableMap) -> Instruction;
}
