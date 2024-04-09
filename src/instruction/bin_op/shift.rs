use super::{LShift, RShift};
use crate::instruction::array::Array;
use crate::instruction::array_repeat::ArrayRepeat;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::variable::{Type, Typed, Variable};
use crate::ExecError;
use duplicate::duplicate_item;

#[duplicate_item(
    shift op1 op2;
    [LShift] [lhs << rhs] [>>]; [RShift] [lhs >> rhs] [>>];
)]
impl shift {
    pub fn create_from_instructions(
        lhs: Instruction,
        rhs: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (lhs, rhs) {
            (Instruction::Variable(_, lhs), Instruction::Variable(_, rhs)) => {
                Ok(Self::exec(lhs, rhs)?.into())
            }
            (_, Instruction::Variable(_, Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(ExecError::OverflowShift)
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

    pub fn exec(lhs: Variable, rhs: Variable) -> Result<Variable, ExecError> {
        match (lhs, rhs) {
            (_, Variable::Int(rhs)) if !(0..=63).contains(&rhs) => Err(ExecError::OverflowShift),
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((op1).into()),
            (var @ Variable::Array(_), _) | (_, var @ Variable::Array(_))
                if var.as_type() == Type::EmptyArray =>
            {
                Ok(var)
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
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                stringify!(op2)
            ),
        }
    }
}
