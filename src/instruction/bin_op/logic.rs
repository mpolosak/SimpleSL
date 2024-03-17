use crate::instruction::{traits::CanBeUsed, Instruction};
use crate::{
    variable::{ReturnType, Type, Typed, Variable},
    Error,
};
use duplicate::duplicate_item;
use std::iter;

#[duplicate_item(T; [And]; [Or])]
#[derive(Debug)]
pub struct T {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

#[duplicate_item(logic symbol cond dv; [And] [&&] [value!=0] [0]; [Or] [||] [value==0] [1])]
impl logic {
    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, stringify!(symbol), rhs_type));
        }
        Ok(Self::create_from_instructions(lhs, rhs))
    }

    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if cond =>
            {
                instruction
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (result, Variable::Int(value)) | (Variable::Int(value), result) if cond => result,
            (Variable::Array(array), _) | (_, Variable::Array(array)) => {
                iter::repeat(Variable::Int(dv)).take(array.len()).collect()
            }
            _ => Variable::Int(dv),
        }
    }
}
