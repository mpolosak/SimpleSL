use super::can_be_used;
use crate::instruction::traits::{BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{local_variable::LocalVariables, CreateInstruction, Exec, Instruction};
use crate::variable::{GetReturnType, Type};
use crate::{interpreter::Interpreter, parse::Rule, variable::Variable, Error, Result};
use pest::iterators::Pair;

#[derive(Debug)]
pub struct Greater {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Greater {
    const SYMBOL: &'static str = ">";

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

impl CanBeUsed for Greater {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        can_be_used(lhs, rhs)
    }
}

impl CreateInstruction for Greater {
    fn create_instruction(
        pair: Pair<Rule>,
        interpreter: &Interpreter,
        local_variables: &mut LocalVariables,
    ) -> Result<Instruction> {
        let mut inner = pair.into_inner();
        let pair = inner.next().unwrap();
        let lhs = Instruction::new(pair, interpreter, local_variables)?;
        let pair = inner.next().unwrap();
        let rhs = Instruction::new(pair, interpreter, local_variables)?;
        let lhs_type = lhs.get_return_type();
        let rhs_type = rhs.get_return_type();
        if Self::can_be_used(&lhs_type, &rhs_type) {
            Self::create_from_instructions(lhs, rhs)
        } else {
            Err(Error::CannotDo2(lhs_type, Self::SYMBOL, rhs_type))
        }
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
            (lhs, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::greater(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array, _), rhs) => array
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

impl GetReturnType for Greater {
    fn get_return_type(&self) -> Type {
        if matches!(
            (self.lhs.get_return_type(), self.rhs.get_return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}

impl From<Greater> for Instruction {
    fn from(value: Greater) -> Self {
        Self::Greater(value.into())
    }
}
