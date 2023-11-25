use crate::instruction::traits::{BaseInstruction, BinOp, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::{Type, Typed};
use crate::{interpreter::Interpreter, variable::Variable, Error, Result};

use super::BitwiseBinOp;

#[derive(Debug)]
pub struct RShift {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for RShift {
    const SYMBOL: &'static str = ">>";

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

impl BitwiseBinOp for RShift {}

impl CreateFromInstructions for RShift {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::rshift(lhs, rhs)?.into())
            }
            (_, Instruction::Variable(Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(Error::OverflowShift)
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl RShift {
    fn rshift(lhs: Variable, rhs: Variable) -> Result<Variable> {
        match (lhs, rhs) {
            (_, Variable::Int(rhs)) if !(0..=63).contains(&rhs) => Err(Error::OverflowShift),
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((lhs >> rhs).into()),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                Ok(array)
            }
            (value, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|element| Self::rshift(value.clone(), element))
                .collect(),
            (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::rshift(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                Self::SYMBOL
            ),
        }
    }
}

impl Exec for RShift {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Self::rshift(lhs, rhs)
    }
}

impl BaseInstruction for RShift {}
