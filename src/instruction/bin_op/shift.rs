use std::sync::Arc;

use super::{LShift, RShift};
use crate::instruction::array::Array;
use crate::instruction::array_repeat::ArrayRepeat;
use crate::instruction::Instruction;
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
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::exec(lhs, rhs)?.into())
            }
            (_, Instruction::Variable(Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(ExecError::OverflowShift)
            }
            (Instruction::Array(array), rhs) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(|iws| iws.try_map(|lhs| Self::create_from_instructions(lhs, rhs.clone())))
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
                    .map(|iws| iws.try_map(|rhs| Self::create_from_instructions(lhs.clone(), rhs)))
                    .collect::<Result<_, _>>()?;
                Ok(Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into())
            }
            (Instruction::ArrayRepeat(array_repeat), rhs) => {
                let array_repeat = Arc::unwrap_or_clone(array_repeat);
                let value = array_repeat
                    .value
                    .try_map(|lhs| Self::create_from_instructions(lhs, rhs))?;
                Ok(ArrayRepeat {
                    value,
                    len: array_repeat.len,
                }
                .into())
            }
            (lhs, Instruction::ArrayRepeat(array_repeat)) => {
                let array_repeat = Arc::unwrap_or_clone(array_repeat);
                let value = array_repeat
                    .value
                    .try_map(|rhs| Self::create_from_instructions(lhs, rhs))?;
                Ok(ArrayRepeat {
                    value,
                    len: array_repeat.len,
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
