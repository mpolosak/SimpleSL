use crate::instruction::{traits::CanBeUsed, Instruction};
use crate::variable::{ReturnType, Variable};
use crate::{Error, ExecError};
use duplicate::duplicate_item;

#[duplicate_item(T; [Divide]; [Modulo])]
#[derive(Debug)]
pub struct T {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

#[duplicate_item(T error operation symbol;
    [Divide] [ZeroDivision] [dividend / divisor] [/];
    [Modulo] [ZeroModulo] [dividend % divisor] [%])]
impl T {
    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, stringify!(symbol), rhs_type));
        }
        Self::create_from_instructions(lhs, rhs).map_err(Error::from)
    }

    pub fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (dividend, divisor) {
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::exec(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(Variable::Int(0))) => Err(ExecError::error),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    pub fn exec(dividend: Variable, divisor: Variable) -> Result<Variable, ExecError> {
        match (dividend, divisor) {
            (_, Variable::Int(0)) => Err(ExecError::error),
            (Variable::Int(dividend), Variable::Int(divisor)) => Ok((operation).into()),
            (Variable::Float(dividend), Variable::Float(divisor)) => {
                Ok((dividend / divisor).into())
            }
            (Variable::Array(array), divisor @ (Variable::Int(_) | Variable::Float(_))) => array
                .iter()
                .cloned()
                .map(|dividend| Self::exec(dividend, divisor.clone()))
                .collect::<Result<Variable, ExecError>>(),
            (dividend @ (Variable::Int(_) | Variable::Float(_)), Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|divisor| Self::exec(dividend.clone(), divisor))
                .collect::<Result<Variable, ExecError>>(),
            (dividend, divisor) => {
                panic!("Tried to calc {dividend} {} {divisor}", stringify!(symbol))
            }
        }
    }
}
