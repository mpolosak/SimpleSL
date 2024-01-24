use crate::binNumOp;
use crate::instruction::Instruction;
use crate::variable::Typed;
use crate::{variable::Variable, Result};

binNumOp!(Subtract, "-");

impl Subtract {
    fn create_from_instructions(
        minuend: Instruction,
        subtrahend: Instruction,
    ) -> Result<Instruction> {
        match (minuend, subtrahend) {
            (Instruction::Variable(minuend), Instruction::Variable(rhs)) => {
                Self::exec(minuend, rhs).map(Instruction::from)
            }
            (minuend, subtrahend) => Ok(Self::construct(minuend, subtrahend).into()),
        }
    }

    fn exec(minuend: Variable, subtrahend: Variable) -> Result<Variable> {
        match (minuend, subtrahend) {
            (Variable::Int(lhs), Variable::Int(rhs)) => Ok((lhs - rhs).into()),
            (Variable::Float(lhs), Variable::Float(rhs)) => Ok((lhs - rhs).into()),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                Ok(array)
            }
            (minuend, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|subtrahend| Self::exec(minuend.clone(), subtrahend))
                .collect(),
            (Variable::Array(array), subtrahend) => array
                .iter()
                .cloned()
                .map(|minuend| Self::exec(minuend, subtrahend.clone()))
                .collect(),
            (minuend, subtrahend) => {
                panic!("Tried to calc {minuend} {} {subtrahend}", Self::SYMBOL)
            }
        }
    }
}
