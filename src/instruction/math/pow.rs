use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::{
    interpreter::Interpreter,
    variable::{ReturnType, Type, Variable},
    Error, Result,
};

#[derive(Debug)]
pub struct Pow {
    base: Instruction,
    exp: Instruction,
}

impl BinOp for Pow {
    const SYMBOL: &'static str = "**";

    fn lhs(&self) -> &Instruction {
        &self.base
    }

    fn rhs(&self) -> &Instruction {
        &self.exp
    }

    fn construct(base: Instruction, exp: Instruction) -> Self {
        Self { base, exp }
    }
}

impl CanBeUsed for Pow {
    fn can_be_used(base: &Type, exp: &Type) -> bool {
        match (base, exp) {
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

impl CreateFromInstructions for Pow {
    fn create_from_instructions(base: Instruction, exp: Instruction) -> Result<Instruction> {
        match (base, exp) {
            (Instruction::Variable(base), Instruction::Variable(exp)) => {
                Ok(Self::pow(base, exp)?.into())
            }
            (_, Instruction::Variable(Variable::Int(exp))) if exp < 0 => {
                Err(Error::CannotBeNegative("exponent"))
            }
            (base, exp) => Ok(Self::construct(base, exp).into()),
        }
    }
}

impl Pow {
    fn pow(base: Variable, exp: Variable) -> Result<Variable> {
        match (base, exp) {
            (_, Variable::Int(exp)) if exp < 0 => Err(Error::CannotBeNegative("exponent")),
            (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
            (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
            (array @ Variable::Array(_, Type::EmptyArray), _)
            | (_, array @ Variable::Array(_, Type::EmptyArray)) => Ok(array),
            (value, Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|element| Self::pow(value.clone(), element))
                .collect(),
            (Variable::Array(array, _), value) => array
                .iter()
                .cloned()
                .map(|element| Self::pow(element, value.clone()))
                .collect(),
            (base, exp) => panic!("Tried to calc {base} {} {exp}", Self::SYMBOL),
        }
    }
}

impl Exec for Pow {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let base = self.base.exec(interpreter)?;
        let exp = self.exp.exec(interpreter)?;
        Pow::pow(base, exp)
    }
}

impl ReturnType for Pow {
    fn return_type(&self) -> Type {
        match (self.base.return_type(), self.exp.return_type()) {
            (_, var_type @ Type::Array(_)) | (var_type, _) => var_type,
        }
    }
}

impl BaseInstruction for Pow {}
