use crate::instruction::array::Array;
use crate::instruction::array_repeat::ArrayRepeat;
use crate::instruction::{Instruction, InstructionWithStr, Pow};
use crate::variable::{Type, Typed, Variable};
use crate::ExecError;

impl Pow {
    pub fn create_from_instructions(
        base: Instruction,
        exp: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (base, exp) {
            (Instruction::Variable(_, base), Instruction::Variable(_, exp)) => {
                Ok(Self::exec(base, exp)?.into())
            }
            (_, Instruction::Variable(_, Variable::Int(exp))) if exp < 0 => {
                Err(ExecError::NegativeExponent)
            }
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
                let value =
                    Self::create_from_instructions(array_repeat.value.instruction.clone(), rhs)?;
                let value = InstructionWithStr {
                    instruction: value,
                    str: array_repeat.value.str.clone(),
                };
                Ok(ArrayRepeat {
                    value,
                    len: array_repeat.len.clone(),
                }
                .into())
            }
            (lhs, Instruction::ArrayRepeat(array_repeat)) => {
                let value =
                    Self::create_from_instructions(lhs, array_repeat.value.instruction.clone())?;
                let value = InstructionWithStr {
                    instruction: value,
                    str: array_repeat.value.str.clone(),
                };
                Ok(ArrayRepeat {
                    value,
                    len: array_repeat.len.clone(),
                }
                .into())
            }
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    pub fn exec(base: Variable, exp: Variable) -> Result<Variable, ExecError> {
        match (base, exp) {
            (_, Variable::Int(exp)) if exp < 0 => Err(ExecError::NegativeExponent),
            (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
            (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                Ok(array.clone())
            }
            (value, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|element| Self::exec(value.clone(), element))
                .collect(),
            (Variable::Array(array), value) => array
                .iter()
                .cloned()
                .map(|element| Self::exec(element, value.clone()))
                .collect(),
            (base, exp) => panic!("Tried to calc {base} * {exp}"),
        }
    }
}
