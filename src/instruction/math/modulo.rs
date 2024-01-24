use crate::binIntOp;
use crate::instruction::Instruction;
use crate::{variable::Variable, Error, Result};

binIntOp!(Modulo, "%");

impl Modulo {
    fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::exec(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroModulo),
            (dividend, divisor) => Ok(Self::construct(dividend, divisor).into()),
        }
    }

    fn exec(dividend: Variable, divisor: Variable) -> Result<Variable> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(Error::ZeroModulo),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend % divisor).into()),
            (Variable::Array(array), divisor @ Variable::Int(_)) => array
                .iter()
                .cloned()
                .map(|dividend| Self::exec(dividend, divisor.clone()))
                .collect::<Result<Variable>>(),
            (dividend @ Variable::Int(_), Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|divisor| Self::exec(dividend.clone(), divisor))
                .collect::<Result<Variable>>(),
            (dividend, divisor) => panic!("Tried to calc {dividend} {} {divisor}", Self::SYMBOL),
        }
    }
}
