use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::Typed;
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub struct Subtract {
    minuend: Instruction,
    subtrahend: Instruction,
}

impl BinOp for Subtract {
    const SYMBOL: &'static str = "-";

    fn lhs(&self) -> &Instruction {
        &self.minuend
    }

    fn rhs(&self) -> &Instruction {
        &self.subtrahend
    }

    fn construct(minuend: Instruction, subtrahend: Instruction) -> Self {
        Self {
            minuend,
            subtrahend,
        }
    }
}

impl CanBeUsed for Subtract {
    fn can_be_used(minuend: &Type, subtrahend: &Type) -> bool {
        match (minuend, subtrahend) {
            (Type::Int, Type::Int)
            | (Type::Float, Type::Float)
            | (Type::EmptyArray, Type::Int | Type::Float)
            | (Type::Int | Type::Float, Type::EmptyArray) => true,
            (Type::Array(element_type), var_type @ (Type::Int | Type::Float))
            | (var_type @ (Type::Int | Type::Float), Type::Array(element_type)) => {
                element_type.as_ref() == var_type
            }
            _ => false,
        }
    }
}

impl CreateFromInstructions for Subtract {
    fn create_from_instructions(
        minuend: Instruction,
        subtrahend: Instruction,
    ) -> Result<Instruction> {
        match (minuend, subtrahend) {
            (Instruction::Variable(minuend), Instruction::Variable(rhs)) => {
                Ok(Self::subtract(minuend, rhs).into())
            }
            (minuend, subtrahend) => Ok(Self::construct(minuend, subtrahend).into()),
        }
    }
}

impl Subtract {
    fn subtract(minuend: Variable, subtrahend: Variable) -> Variable {
        match (minuend, subtrahend) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs - rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs - rhs).into(),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (minuend, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|subtrahend| Self::subtract(minuend.clone(), subtrahend))
                .collect(),
            (Variable::Array(array), subtrahend) => array
                .iter()
                .cloned()
                .map(|minuend| Self::subtract(minuend, subtrahend.clone()))
                .collect(),
            (minuend, subtrahend) => {
                panic!("Tried to calc {minuend} {} {subtrahend}", Self::SYMBOL)
            }
        }
    }
}

impl Exec for Subtract {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let minuend = self.minuend.exec(interpreter)?;
        let subtrahend = self.subtrahend.exec(interpreter)?;
        Ok(Self::subtract(minuend, subtrahend))
    }
}

impl ReturnType for Subtract {
    fn return_type(&self) -> Type {
        match (self.minuend.return_type(), self.subtrahend.return_type()) {
            (_, var_type @ Type::Array(_)) | (var_type, _) => var_type,
        }
    }
}

impl BaseInstruction for Subtract {}
