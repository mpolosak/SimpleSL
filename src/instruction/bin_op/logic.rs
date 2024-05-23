use super::{And, Or};
use crate::instruction::Instruction;
use crate::variable::{Array, Variable};
use duplicate::duplicate_item;
use std::iter;

#[duplicate_item(logic symbol cond dv; [And] [&&] [value!=0] [0]; [Or] [||] [value==0] [1])]
impl logic {
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
            (result, Variable::Int(value)) | (Variable::Int(value), result) if cond => result,
            (Variable::Array(array), _) | (_, Variable::Array(array)) => {
                let elements = iter::repeat(Variable::Int(dv)).take(array.len()).collect();
                let var_type = array.var_type.clone();
                Array { var_type, elements }.into()
            }
            _ => Variable::Int(dv),
        }
    }
}
