pub mod add;
mod divide;
mod pow;
pub mod subtract;
use super::{Greater, GreaterOrEqual, Lower, LowerOrEqual, Multiply};
use crate::variable::{Array, Variable};
use duplicate::duplicate_item;
use match_any::match_any;

#[duplicate_item(
    ord op;
    [Multiply] [*]; [Greater] [>]; [GreaterOrEqual] [>=]; [Lower] [<]; [LowerOrEqual] [<=];
)]
impl ord {
    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match_any! { (lhs, rhs),
            (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
                => (lhs op rhs).into(),
            (lhs, Variable::Array(array)) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|rhs| Self::exec(lhs.clone(), rhs))
                    .collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            },
            (Variable::Array(array), rhs) => {
                let elements = array
                    .iter()
                    .cloned()
                    .map(|lhs| Self::exec(lhs, rhs.clone()))
                    .collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            },
            (lhs, rhs) => panic!("Tried to do {lhs} {} {rhs}", stringify!(op))
        }
    }
}
