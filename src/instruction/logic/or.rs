use crate::binIntOp;
use crate::instruction::traits::CreateFromInstructions;
use crate::instruction::Instruction;
use crate::variable::Typed;
use crate::{variable::Variable, Result};

binIntOp!(Or, "||");

impl CreateFromInstructions for Or {
    fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => {
                Ok(Self::or(lhs, rhs).into())
            }
            (Instruction::Variable(Variable::Int(value)), instruction)
            | (instruction, Instruction::Variable(Variable::Int(value)))
                if value == 0 =>
            {
                Ok(instruction)
            }
            (lhs, rhs) => Ok(Self::construct(lhs, rhs).into()),
        }
    }
}

impl Or {
    fn exec(lhs: Variable, rhs: Variable) -> Result<Variable> {
        Ok(Self::or(lhs, rhs))
    }
    fn or(lhs: Variable, rhs: Variable) -> Variable {
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
            (lhs, rhs) => panic!("Tried {lhs} {} {rhs} which is imposible", Self::SYMBOL),
        }
    }
}
