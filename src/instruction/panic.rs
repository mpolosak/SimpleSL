use crate::{instruction::{BaseInstruction, Exec, Recreate}, variable::{ReturnType, Type}};

use super::{local_variable::LocalVariables, ExecResult, Instruction};

#[derive(Debug)]
pub struct Panic;

impl BaseInstruction for Panic{}

impl ReturnType for Panic{
    fn return_type(&self) -> Type {
        Type::Never
    }
}

impl Exec for Panic{
    fn exec(&self, _interpreter: &mut crate::Interpreter) -> ExecResult {
        unreachable!()
    }
}

impl Recreate for Panic{
    fn recreate(&self, _local_variables: &mut LocalVariables) -> Instruction {
        unreachable!()
    }
}