use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::{ReturnType, Type};
use crate::{interpreter::Interpreter, variable::Variable, Result};

use super::can_be_used;

#[derive(Debug)]
pub struct LowerOrEqual {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for LowerOrEqual {
    const SYMBOL: &'static str = "<=";

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

impl CanBeUsed for LowerOrEqual {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        can_be_used(lhs, rhs)
    }
}

impl CreateFromInstructions for LowerOrEqual {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::greater_or_equal(lhs, rhs).into())
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl LowerOrEqual {
    fn greater_or_equal(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs <= rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs <= rhs).into(),
            (lhs, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::greater_or_equal(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array, _), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::greater_or_equal(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} {} {rhs}", Self::SYMBOL),
        }
    }
}

impl Exec for LowerOrEqual {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::greater_or_equal(lhs, rhs))
    }
}

impl ReturnType for LowerOrEqual {
    fn return_type(&self) -> Type {
        if matches!(
            (self.lhs.return_type(), self.rhs.return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}

impl BaseInstruction for LowerOrEqual {}
