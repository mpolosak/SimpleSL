use crate::instruction::macros::bin_num_op::ACCEPTED_TYPE;
use crate::instruction::{macros::binIntOp, Instruction};
use crate::variable::Variable;
use crate::variable::{Type, Typed};
use crate::ExecError;
use duplicate::duplicate_item;

binIntOp!(LShift, "<<");
binIntOp!(RShift, ">>");

#[duplicate_item(
    shift op1 op2;
    [LShift] [lhs << rhs] [>>]; [RShift] [lhs >> rhs] [>>];
)]
impl shift {
    fn create_from_instructions(
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
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable, ExecError> {
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
