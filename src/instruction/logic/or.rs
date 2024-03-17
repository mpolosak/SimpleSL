use crate::instruction::macros::ACCEPTED_INT_TYPE as ACCEPTED_TYPE;
use crate::instruction::{macros::binOpCBU, Instruction};
use crate::variable::Variable;
use crate::variable::{Type, Typed};

binOpCBU!(Or, "||");

impl Or {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(0)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(0))) => instruction,
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
            (value, Variable::Int(0)) | (Variable::Int(0), value) => value,
            (Variable::Int(_), Variable::Int(_)) => Variable::Int(1),
            (Variable::Array(array), Variable::Int(_))
            | (Variable::Int(_), Variable::Array(array)) => std::iter::repeat(Variable::Int(1))
                .take(array.len())
                .collect(),
            (lhs, rhs) => panic!("Tried {lhs} || {rhs} which is imposible"),
        }
    }
}
