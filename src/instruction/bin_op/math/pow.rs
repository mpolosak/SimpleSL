use crate::instruction::{Instruction, Pow};
use crate::variable::Variable;
use crate::ExecError;
use std::sync::Arc;

impl Pow {
    pub fn create_from_instructions(
        base: Instruction,
        exp: Instruction,
    ) -> Result<Instruction, ExecError> {
        match (base, exp) {
            (Instruction::Variable(base), Instruction::Variable(exp)) => {
                Ok(Self::exec(base, exp)?.into())
            }
            (_, Instruction::Variable(Variable::Int(exp))) if exp < 0 => {
                Err(ExecError::NegativeExponent)
            }
            (Instruction::ArrayRepeat(array), rhs) => Arc::unwrap_or_clone(array)
                .try_map(|lhs| Self::create_from_instructions(lhs, rhs.clone()))
                .map(Instruction::from),
            (lhs, Instruction::ArrayRepeat(array)) => Arc::unwrap_or_clone(array)
                .try_map(|rhs| Self::create_from_instructions(lhs.clone(), rhs))
                .map(Instruction::from),
            (lhs, rhs) => Ok(Self { lhs, rhs }.into()),
        }
    }

    pub fn exec(base: Variable, exp: Variable) -> Result<Variable, ExecError> {
        match (base, exp) {
            (_, Variable::Int(exp)) if exp < 0 => Err(ExecError::NegativeExponent),
            (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
            (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
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
            (base, exp) => panic!("Tried to calc {base} * {exp}"),
        }
    }
}
