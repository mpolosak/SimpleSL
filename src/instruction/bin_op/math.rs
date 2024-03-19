mod add;
mod divide;
mod pow;
use super::{Greater, GreaterOrEqual, Lower, LowerOrEqual, Multiply, Subtract};
use crate::variable::Variable;
use duplicate::duplicate_item;
use match_any::match_any;

#[duplicate_item(
    ord op;
    [Multiply] [*]; [Subtract] [-]; [Greater] [>]; [GreaterOrEqual] [>=]; [Lower] [<]; [LowerOrEqual] [<=];
)]
impl ord {
    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        match_any! { (lhs, rhs),
            (Variable::Int(lhs), Variable::Int(rhs)) | (Variable::Float(lhs), Variable::Float(rhs))
                => (lhs op rhs).into(),
            (lhs, Variable::Array(array)) => array
                .iter()
                .cloned()
                .map(|rhs| Self::exec(lhs.clone(), rhs))
                .collect(),
            (Variable::Array(array), rhs) => array
                .iter()
                .cloned()
                .map(|lhs| Self::exec(lhs, rhs.clone()))
                .collect(),
            (lhs, rhs) => panic!("Tried to do {lhs} {} {rhs}", stringify!(op))
        }
    }
}
