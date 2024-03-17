use crate::instruction::traits::{CanBeUsed, ExecResult};
use crate::instruction::{macros::binOp, Exec, Instruction};
use crate::interpreter::Interpreter;
use crate::variable::{ReturnType, Type};

binOp!(Equal, "==");

impl CanBeUsed for Equal {
    fn can_be_used(_: &Type, _: &Type) -> bool {
        true
    }
}

impl Equal {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(variable), Instruction::Variable(variable2)) => {
                Instruction::Variable((variable == variable2).into())
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

impl Exec for Equal {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok((lhs == rhs).into())
    }
}

impl ReturnType for Equal {
    fn return_type(&self) -> Type {
        Type::Int
    }
}
