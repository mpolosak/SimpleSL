use crate::variable::Type;

pub fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    (lhs == &Type::Int || lhs == &Type::Bool) && lhs == rhs
}
