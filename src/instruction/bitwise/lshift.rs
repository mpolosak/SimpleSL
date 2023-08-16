use crate::instruction::traits::{BinOp, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::{
    interpreter::Interpreter,
    variable::{Type, Variable},
    Error, Result,
};

use super::BitwiseBinOp;

#[derive(Debug)]
pub struct LShift {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for LShift {
    const SYMBOL: &'static str = "<<";

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

impl BitwiseBinOp for LShift {}

impl CreateFromInstructions for LShift {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::lshift(lhs, rhs)?.into())
            }
            (_, Instruction::Variable(Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(Error::OverflowShift)
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl LShift {
    fn lshift(lhs: Variable, rhs: Variable) -> Result<Variable> {
        match (lhs, rhs) {
            (_, Variable::Int(rhs)) if !(0..=63).contains(&rhs) => Err(Error::OverflowShift),
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((lhs << rhs).into()),
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => Ok(array),
            (value, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|element| Self::lshift(value.clone(), element))
                .collect(),
            (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::lshift(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
        }
    }
}

impl Exec for LShift {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Self::lshift(lhs, rhs)
    }
}

impl From<LShift> for Instruction {
    fn from(value: LShift) -> Self {
        Self::LShift(value.into())
    }
}
