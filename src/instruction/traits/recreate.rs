use crate::{
    errors::ExecError,
    instruction::{
        local_variable::LocalVariables,
        prefix_op::{BitwiseNot, Not, UnaryMinus},
        Instruction,
    },
};
use duplicate::duplicate_item;

pub trait Recreate {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError>;
}

#[duplicate_item(T; [UnaryMinus]; [BitwiseNot]; [Not])]
impl Recreate for T {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        Ok(Self::create_from_instruction(instruction))
    }
}
