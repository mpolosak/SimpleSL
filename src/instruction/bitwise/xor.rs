use crate::instruction::traits::{BaseInstruction, BinOp, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::{
    interpreter::Interpreter,
    variable::{Type, Variable},
    Result,
};

use super::BitwiseBinOp;

#[derive(Debug)]
pub struct Xor {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Xor {
    const SYMBOL: &'static str = "^";

    fn get_lhs(&self) -> &Instruction {
        &self.lhs
    }

    fn get_rhs(&self) -> &Instruction {
        &self.rhs
    }

    fn construct(lhs: Instruction, rhs: Instruction) -> Self {
        Self { lhs, rhs }
    }
}

impl BitwiseBinOp for Xor {}

impl CreateFromInstructions for Xor {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::xor(lhs, rhs).into())
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl Xor {
    fn xor(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs ^ rhs).into(),
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => array,
            (value, Variable::Array(array, _)) | (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::xor(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
        }
    }
}

impl Exec for Xor {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::xor(lhs, rhs))
    }
}

impl BaseInstruction for Xor {}
