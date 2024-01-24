use crate::binNumOp;
use crate::instruction::Instruction;
use crate::variable::Typed;
use crate::{variable::Variable, Error, Result};

binNumOp!(Pow, "*");

impl Pow {
    fn create_from_instructions(base: Instruction, exp: Instruction) -> Result<Instruction> {
        match (base, exp) {
            (Instruction::Variable(base), Instruction::Variable(exp)) => {
                Ok(Self::exec(base, exp)?.into())
            }
            (_, Instruction::Variable(Variable::Int(exp))) if exp < 0 => {
                Err(Error::CannotBeNegative("exponent"))
            }
            (base, exp) => Ok(Self::construct(base, exp).into()),
        }
    }

    fn exec(base: Variable, exp: Variable) -> Result<Variable> {
        match (base, exp) {
            (_, Variable::Int(exp)) if exp < 0 => Err(Error::CannotBeNegative("exponent")),
            (Variable::Int(base), Variable::Int(exp)) => Ok((base.pow(exp as u32)).into()),
            (Variable::Float(base), Variable::Float(exp)) => Ok((base.powf(exp)).into()),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                Ok(array.clone())
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
            (base, exp) => panic!("Tried to calc {base} {} {exp}", Self::SYMBOL),
        }
    }
}
