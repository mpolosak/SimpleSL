use crate::variable::Type;

pub trait CanBeUsed {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool;
}
