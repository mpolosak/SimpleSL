use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::{
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Error, Result,
};

#[derive(Debug)]
pub struct Modulo {
    dividend: Instruction,
    divisor: Instruction,
}

impl BinOp for Modulo {
    const SYMBOL: &'static str = "%";

    fn get_lhs(&self) -> &Instruction {
        &self.dividend
    }

    fn get_rhs(&self) -> &Instruction {
        &self.divisor
    }

    fn construct(dividend: Instruction, divisor: Instruction) -> Self {
        Self { dividend, divisor }
    }
}

impl CanBeUsed for Modulo {
    fn can_be_used(dividend: &Type, divisor: &Type) -> bool {
        match (dividend, divisor) {
            (Type::Int | Type::EmptyArray, Type::Int) | (Type::Int, Type::EmptyArray) => true,
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type)) => {
                var_type.as_ref() == &Type::Int
            }
            _ => false,
        }
    }
}

impl CreateFromInstructions for Modulo {
    fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::modulo(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroModulo),
            (dividend, divisor) => Ok(Self::construct(dividend, divisor).into()),
        }
    }
}

impl Modulo {
    fn modulo(dividend: Variable, divisor: Variable) -> Result<Variable> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(Error::ZeroModulo),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend % divisor).into()),
            (Variable::Array(array, _), divisor @ Variable::Int(_)) => array
                .iter()
                .cloned()
                .map(|dividend| Self::modulo(dividend, divisor.clone()))
                .collect::<Result<Variable>>(),
            (dividend @ Variable::Int(_), Variable::Array(array, _)) => array
                .iter()
                .cloned()
                .map(|divisor| Self::modulo(dividend.clone(), divisor))
                .collect::<Result<Variable>>(),
            (dividend, divisor) => panic!("Tried to calc {dividend} {} {divisor}", Self::SYMBOL),
        }
    }
}

impl Exec for Modulo {
    fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        let dividend = self.dividend.exec(interpreter)?;
        let divisor = self.divisor.exec(interpreter)?;
        Self::modulo(dividend, divisor)
    }
}

impl GetReturnType for Modulo {
    fn get_return_type(&self) -> Type {
        if matches!(
            (
                self.dividend.get_return_type(),
                self.divisor.get_return_type()
            ),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}

impl BaseInstruction for Modulo {}
