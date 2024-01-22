use super::can_be_used;
use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::{ReturnType, Type};
use crate::{interpreter::Interpreter, variable::Variable, Result};

#[derive(Debug)]
pub struct Greater {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Greater {
    const SYMBOL: &'static str = ">";

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

impl CanBeUsed for Greater {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        can_be_used(lhs, rhs)
    }
}

impl CreateFromInstructions for Greater {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::greater(lhs, rhs).into())
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl Greater {
    fn greater(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs > rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs > rhs).into(),
            (lhs, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::greater(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::greater(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} {} {rhs}", Self::SYMBOL),
        }
    }
}

impl Exec for Greater {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::greater(lhs, rhs))
    }
}

impl ReturnType for Greater {
    fn return_type(&self) -> Type {
        if matches!(
            (self.lhs.return_type(), self.rhs.return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            [Type::Int].into()
        } else {
            Type::Int
        }
    }
}

impl BaseInstruction for Greater {}
