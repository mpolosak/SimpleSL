use crate::instruction::array::Array;
use crate::instruction::array_repeat::ArrayRepeat;
use crate::instruction::{Divide, Modulo};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::variable::Variable;
use crate::ExecError;
use duplicate::duplicate_item;

#[duplicate_item(T error operation symbol;
    [Divide] [ZeroDivision] [dividend / divisor] [/];
    [Modulo] [ZeroModulo] [dividend % divisor] [%])]
impl T {
    pub fn create_from_instructions(
        dividend: Instruction,
        divisor: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (dividend, divisor) {
            (Instruction::Variable(_, dividend), Instruction::Variable(_, divisor)) => {
                Ok(Self::exec(dividend, divisor)?.into())
            }
            (_, Instruction::Variable(_, Variable::Int(0))) => Err(ExecError::error),
            (Instruction::Array(array), rhs) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(
                        |InstructionWithStr {
                             instruction: lhs,
                             str,
                         }| {
                            let instruction = Self::create_from_instructions(lhs, rhs.clone())?;
                            Ok(InstructionWithStr { instruction, str })
                        },
                    )
                    .collect::<Result<_, _>>()?;
                Ok(Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into())
            }
            (lhs, Instruction::Array(array)) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(
                        |InstructionWithStr {
                             instruction: rhs,
                             str,
                         }| {
                            let instruction = Self::create_from_instructions(lhs.clone(), rhs)?;
                            Ok(InstructionWithStr { instruction, str })
                        },
                    )
                    .collect::<Result<_, _>>()?;
                Ok(Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into())
            }
            (Instruction::ArrayRepeat(array_repeat), rhs) => {
                let value = Self::create_from_instructions(array_repeat.value.clone(), rhs)?;
                Ok(ArrayRepeat {
                    value,
                    len: array_repeat.len.clone(),
                }
                .into())
            }
            (lhs, Instruction::ArrayRepeat(array_repeat)) => {
                let value = Self::create_from_instructions(lhs, array_repeat.value.clone())?;
                Ok(ArrayRepeat {
                    value,
                    len: array_repeat.len.clone(),
                }
                .into())
            }
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
