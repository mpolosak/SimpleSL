use duplicate::duplicate_item;
#[duplicate_item(
    shift Shift op1 op2;
    [lshift] [LShift] [lhs << rhs] [>>]; [rshift] [RShift] [lhs >> rhs] [>>];
)]
pub mod shift {
    use std::sync::Arc;

    use crate::{
        instruction::{
            traits::can_be_used_int, BinOperation, BinOperator, Instruction, InstructionWithStr,
        },
        variable::{Array, ReturnType, Variable},
        Error, ExecError,
    };

    pub fn create_op(
        lhs: InstructionWithStr,
        rhs: InstructionWithStr,
    ) -> Result<InstructionWithStr, Error> {
        let str = format!("{} {} {}", lhs.str, stringify!(op), rhs.str).into();
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !can_be_used_int(lhs_type.clone(), rhs_type.clone()) {
            return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
        }
        let instruction = create_from_instructions(lhs.instruction, rhs.instruction)?;
        Ok(InstructionWithStr { instruction, str })
    }

    pub fn create_from_instructions(
        lhs: Instruction,
        rhs: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Ok(exec(lhs, rhs)?.into()),
            (_, Instruction::Variable(Variable::Int(rhs))) if !(0..=63).contains(&rhs) => {
                Err(ExecError::OverflowShift)
            }
            (Instruction::ArrayRepeat(array), rhs) => Arc::unwrap_or_clone(array)
                .try_map(|lhs| create_from_instructions(lhs, rhs.clone()))
                .map(Instruction::from),
            (lhs, Instruction::ArrayRepeat(array)) => Arc::unwrap_or_clone(array)
                .try_map(|rhs| create_from_instructions(lhs.clone(), rhs))
                .map(Instruction::from),
            (lhs, rhs) => Ok(BinOperation {
                lhs,
                rhs,
                op: BinOperator::Shift,
            }
            .into()),
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
                    .map(|rhs| exec(value.clone(), rhs))
                    .collect::<Result<Arc<_>, _>>()?;
                let element_type = array.element_type().clone();
                Ok(Array {
                    element_type,
                    elements,
                }
                .into())
            }
            (Variable::Array(array), value) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|lhs| exec(lhs, value.clone()))
                    .collect::<Result<Arc<_>, _>>()?;
                let element_type = array.element_type().clone();
                Ok(Array {
                    element_type,
                    elements,
                }
                .into())
            }
            (lhs, rhs) => panic!(
                "Tried to do {lhs} {} {rhs} which is imposible",
                stringify!(op2)
            ),
        }
    }
}
