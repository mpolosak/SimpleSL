use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::variable::Typed;
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Result,
};

#[derive(Debug)]
pub struct Multiply {
    lhs: Instruction,
    rhs: Instruction,
}

impl BinOp for Multiply {
    const SYMBOL: &'static str = "*";

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

impl CanBeUsed for Multiply {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
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

impl CreateFromInstructions for Multiply {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::multiply(lhs, rhs).into())
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl Multiply {
    fn multiply(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (lhs * rhs).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (lhs * rhs).into(),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (value, Variable::Array(array)) | (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::multiply(element, value.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to calc {lhs} {} {rhs}", Self::SYMBOL),
        }
    }
}

impl Exec for Multiply {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let lhs = self.lhs.exec(interpreter)?;
        let rhs = self.rhs.exec(interpreter)?;
        Ok(Self::multiply(lhs, rhs))
    }
}

impl ReturnType for Multiply {
    fn return_type(&self) -> Type {
        match (self.lhs.return_type(), self.rhs.return_type()) {
            (_, var_type @ Type::Array(_)) | (var_type, _) => var_type,
        }
    }
}

impl BaseInstruction for Multiply {}
