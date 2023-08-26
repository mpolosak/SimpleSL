use crate::instruction::{local_variable::LocalVariables, Instruction};
use crate::interpreter::Interpreter;
use crate::Result;

use super::{BinOp, Recreate};

pub trait CreateFromInstructions {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction>;
}

impl<T: CreateFromInstructions + BinOp> Recreate for T {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction> {
        let lhs = self.lhs().recreate(local_variables, interpreter)?;
        let rhs = self.rhs().recreate(local_variables, interpreter)?;
        Self::create_from_instructions(lhs, rhs)
    }
}
