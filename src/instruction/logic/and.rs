use crate::binIntOp;
use crate::instruction::Instruction;
use crate::variable::Typed;
use crate::{variable::Variable, Result};

binIntOp!(And, "&&");

impl And {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        Ok(match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::and(lhs, rhs).into(),
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value != 0 =>
            {
                instruction
            }
            (lhs, rhs) => Self::construct(lhs, rhs).into(),
        })
    }

    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable> {
        Ok(Self::and(lhs, rhs))
    }

    fn and(lhs: Variable, rhs: Variable) -> Variable {
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
            (lhs, rhs) => panic!("Tried {lhs} {} {rhs} which is imposible", Self::SYMBOL),
        }
    }
}
