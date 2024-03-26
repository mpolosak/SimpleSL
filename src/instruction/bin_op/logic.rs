use super::{And, Or};
use crate::instruction::array::Array;
use crate::instruction::Instruction;
use crate::variable::{Type, Typed, Variable};
use duplicate::duplicate_item;
use std::iter;

#[duplicate_item(logic symbol cond dv; [And] [&&] [value!=0] [0]; [Or] [||] [value==0] [1])]
impl logic {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(_, lhs), Instruction::Variable(_, rhs)) => {
                Self::exec(lhs, rhs).into()
            }
            (Instruction::Variable(_, Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(_, Variable::Int(value)))
                if cond =>
            {
                instruction
            }
            (Instruction::Array(array), rhs) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(|lhs| Self::create_from_instructions(lhs, rhs.clone()))
                    .collect();
                Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into()
            }
            (lhs, Instruction::Array(array)) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(|rhs| Self::create_from_instructions(lhs.clone(), rhs))
                    .collect();
                Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into()
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
