mod add;
mod divide;
mod pow;
use super::{Multiply, Subtract};
use crate::variable::{Type, Typed, Variable};
use duplicate::duplicate_item;

#[duplicate_item(T op symbol; [Multiply] [lhs*rhs] [*]; [Subtract] [lhs-rhs] [-])]
impl T {
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
