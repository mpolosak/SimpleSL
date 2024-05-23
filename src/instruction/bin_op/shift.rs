use super::{LShift, RShift};
use crate::instruction::Instruction;
use crate::variable::{Array, Variable};
use crate::ExecError;
use duplicate::duplicate_item;
use std::sync::Arc;

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
            (Instruction::ArrayRepeat(array), rhs) => Arc::unwrap_or_clone(array)
                .try_map(|lhs| Self::create_from_instructions(lhs, rhs.clone()))
                .map(Instruction::from),
            (lhs, Instruction::ArrayRepeat(array)) => Arc::unwrap_or_clone(array)
                .try_map(|rhs| Self::create_from_instructions(lhs.clone(), rhs))
                .map(Instruction::from),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Result<Variable, ExecError> {
        match (lhs, rhs) {
            (_, Variable::Int(rhs)) if !(0..=63).contains(&rhs) => Err(ExecError::OverflowShift),
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((op1).into()),
            (value, Variable::Array(array)) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|rhs| Self::exec(value.clone(), rhs))
                    .collect::<Result<Arc<_>, _>>()?;
                let var_type = array.var_type.clone();
                Ok(Array { var_type, elements }.into())
            }
            (Variable::Array(array), value) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|lhs| Self::exec(lhs, value.clone()))
                    .collect::<Result<Arc<_>, _>>()?;
                let var_type = array.var_type.clone();
                Ok(Array { var_type, elements }.into())
            }
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                stringify!(op2)
            ),
        }
    }
}
