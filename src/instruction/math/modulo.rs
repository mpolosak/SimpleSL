use crate::instruction::traits::{BaseInstruction, BinIntOp, BinOp, CreateFromInstructions};
use crate::instruction::{Exec, Instruction};
use crate::{interpreter::Interpreter, variable::Variable, Error, Result};

#[derive(Debug)]
pub struct Modulo {
    dividend: Instruction,
    divisor: Instruction,
}

impl BinOp for Modulo {
    const SYMBOL: &'static str = "%";

    fn lhs(&self) -> &Instruction {
        &self.dividend
    }

    fn rhs(&self) -> &Instruction {
        &self.divisor
    }

    fn construct(dividend: Instruction, divisor: Instruction) -> Self {
        Self { dividend, divisor }
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
            (Variable::Array(array), divisor @ Variable::Int(_)) => array
                .iter()
                .cloned()
                .map(|dividend| Self::modulo(dividend, divisor.clone()))
                .collect::<Result<Variable>>(),
            (dividend @ Variable::Int(_), Variable::Array(array)) => array
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

impl BaseInstruction for Modulo {}
impl BinIntOp for Modulo {}
