use crate::instruction::{
    macros::{binIntOp, bin_int_op::ACCEPTED_TYPE},
    Instruction,
};
use crate::variable::Variable;
use crate::variable::{Type, Typed};

binIntOp!(And, "&&");

impl And {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value != 0 =>
            {
                instruction
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }

    fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
            }
            (Variable::Int(_), Variable::Int(0)) | (Variable::Int(0), Variable::Int(_)) => {
                Variable::Int(0)
            }
            (Variable::Array(array), Variable::Int(0))
            | (Variable::Int(0), Variable::Array(array)) => std::iter::repeat(Variable::Int(0))
                .take(array.len())
                .collect(),
            (value, Variable::Int(_)) | (Variable::Int(_), value) => value,
            (lhs, rhs) => panic!("Tried {lhs} && {rhs} which is imposible"),
        }
    }
}
