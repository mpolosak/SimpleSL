use crate::binNumOp;
use crate::instruction::traits::CreateFromInstructions;
use crate::instruction::Instruction;
use crate::{variable::Variable, Error, Result};

binNumOp!(Divide, "/");

impl CreateFromInstructions for Divide {
    fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::exec(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(Error::ZeroDivision),
            (dividend, divisor) => Ok(Self::construct(dividend, divisor).into()),
        }
    }
}

impl Divide {
    fn exec(dividend: Variable, divisor: Variable) -> Result<Variable> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(Error::ZeroDivision),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((dividend / divisor).into()),
            (Variable::Float(dividend), Variable::Float(divisor)) => {
                Ok((dividend / divisor).into())
            }
            (Variable::Array(array), divisor @ (Variable::Int(_) | Variable::Float(_))) => array
                .iter()
                .cloned()
                .map(|dividend| Self::exec(dividend, divisor.clone()))
                .collect::<Result<Variable>>(),
            (dividend @ (Variable::Int(_) | Variable::Float(_)), Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|divisor| Self::exec(dividend.clone(), divisor))
                .collect::<Result<Variable>>(),
            (dividend, divisor) => panic!("Tried to calc {dividend} {} {divisor}", Self::SYMBOL),
        }
    }
}
