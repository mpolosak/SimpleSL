use crate::binOp;
use crate::instruction::traits::CanBeUsed;
use crate::instruction::{Exec, Instruction};
use crate::variable::{ReturnType, Type};
use crate::{interpreter::Interpreter, variable::Variable, Result};

binOp!(Equal, "==");

impl CanBeUsed for Equal {
    fn can_be_used(_lhs: &crate::variable::Type, _rhs: &crate::variable::Type) -> bool {
        true
    }
}

impl Equal {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(variable), Instruction::Variable(variable2)) => {
                Ok(Instruction::Variable((variable == variable2).into()))
            }
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }
}

impl Exec for Equal {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok((lhs == rhs).into())
    }
}

impl ReturnType for Equal {
    fn return_type(&self) -> crate::variable::Type {
        Type::Int
    }
}
