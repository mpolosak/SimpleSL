use duplicate::duplicate_item;

use crate::{
    errors::ExecError,
    instruction::{
        bin_op::*,
        local_variable::LocalVariables,
        prefix_op::{BitwiseNot, Not, UnaryMinus},
        Instruction,
    },
    Interpreter,
};

use super::ToResult;

pub trait Recreate {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError>;
}

#[duplicate_item(T; [UnaryMinus]; [BitwiseNot]; [Not])]
impl Recreate for T {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &Interpreter,
    ) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables, interpreter)?;
        Ok(Self::create_from_instruction(instruction))
    }
}

#[duplicate_item(
    T;
    [Add]; [Subtract]; [Multiply]; [Divide]; [Modulo]; [Pow]; [And]; [Or];
    [BitwiseAnd]; [BitwiseOr]; [Xor]; [Equal]; [Greater]; [GreaterOrEqual];
    [Lower]; [LowerOrEqual]; [Filter]; [Map]; [LShift]; [RShift]
)]
impl Recreate for T {
    fn recreate(
        &self,
        local_variables: &mut LocalVariables,
        interpreter: &crate::Interpreter,
    ) -> Result<Instruction, ExecError> {
        let lhs = self.lhs.recreate(local_variables, interpreter)?;
        let rhs = self.rhs.recreate(local_variables, interpreter)?;
        Self::create_from_instructions(lhs, rhs).to_result()
    }
}
