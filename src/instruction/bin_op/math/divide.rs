use crate::instruction::Instruction;
use crate::instruction::{Divide, Modulo};
use crate::variable::Variable;
use crate::ExecError;
use duplicate::duplicate_item;
use match_any::match_any;
use std::sync::Arc;

#[duplicate_item(T error operation symbol;
    [Divide] [ZeroDivision] [dividend / divisor] [/];
    [Modulo] [ZeroModulo] [dividend % divisor] [%])]
impl T {
    pub fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction, ExecError> {
        match_any! {(dividend, divisor),
            (Instruction::Variable(dividend), Instruction::Variable(divisor)) => {
                Ok(Self::exec(dividend, divisor)?.into())
            },
            (_, Instruction::Variable(Variable::Int(0))) => Err(ExecError::error),
            (Instruction::ArrayRepeat(array), rhs)
            => Arc::unwrap_or_clone(array)
                .try_map(|lhs| Self::create_from_instructions(lhs, rhs.clone()))
                .map(Instruction::from),
            (lhs, Instruction::ArrayRepeat(array))
            => Arc::unwrap_or_clone(array)
                .try_map(|rhs| Self::create_from_instructions(lhs.clone(), rhs))
                .map(Instruction::from),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into())
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
