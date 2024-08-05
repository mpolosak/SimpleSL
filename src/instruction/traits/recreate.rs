use crate::{
    errors::ExecError,
    instruction::{
        bin_op::*,
        local_variable::LocalVariables,
        prefix_op::{BitwiseNot, Not, UnaryMinus},
        Instruction,
    },
    ToResult,
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

#[duplicate_item(
    T; [And]; [Or];
)]
impl Recreate for T {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let lhs = self.lhs.recreate(local_variables)?;
        let rhs = self.rhs.recreate(local_variables)?;
        Self::create_from_instructions(lhs, rhs).to_result()
    }
}
