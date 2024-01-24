use crate::variable::Type;
mod equal;
mod greater;
mod greater_or_equal;
mod lower;
mod lower_or_equal;
use super::macros::bin_num_op::ACCEPTED_TYPE;

pub use {
    equal::Equal, greater::Greater, greater_or_equal::GreaterOrEqual, lower::Lower,
    lower_or_equal::LowerOrEqual,
};

fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    Type::matches(&(lhs.clone(), rhs.clone()).into(), &ACCEPTED_TYPE)
}
