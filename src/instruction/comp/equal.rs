use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::{ReturnType, Type};
use crate::{interpreter::Interpreter, variable::Variable, Result};

#[derive(Debug)]
pub struct Equal {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Equal {
    const SYMBOL: &'static str = "==";

    fn lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn rhs(&self) -> &Instruction {
        &self.rhs
    }

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl CanBeUsed for Equal {
    fn can_be_used(_lhs: &crate::variable::Type, _rhs: &crate::variable::Type) -> bool {
        true
    }
}

impl CreateFromInstructions for Equal {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(variable), Instruction::Variable(variable2)) => {
                Ok(Instruction::Variable((variable == variable2).into()))
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
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

impl BaseInstruction for Equal {}
