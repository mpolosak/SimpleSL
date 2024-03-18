mod add;
mod divide;
mod pow;
use super::binOp;
use crate::{
    instruction::Instruction,
    variable::{Type, Typed, Variable},
};
use duplicate::duplicate_item;
pub use {
    add::Add,
    divide::{Divide, Modulo},
    pow::Pow,
};

binOp!(Multiply, "*");
binOp!(Subtract, "-");

#[duplicate_item(T op symbol; [Multiply] [lhs*rhs] [*]; [Subtract] [lhs-rhs] [-])]
impl T {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }

    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match (lhs, rhs) {
            (Variable::Int(lhs), Variable::Int(rhs)) => (op).into(),
            (Variable::Float(lhs), Variable::Float(rhs)) => (op).into(),
            (array @ Variable::Array(_), _) | (_, array @ Variable::Array(_))
                if array.as_type() == Type::EmptyArray =>
            {
                array
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
            (lhs, rhs) => panic!("Tried to calc {lhs} {} {rhs}", stringify!(symbol)),
        }
    }
}
