use crate::variable::Type;
mod equal;
mod greater;
mod greater_or_equal;
mod lower;
mod lower_or_equal;
pub use {
    equal::Equal, greater::Greater, greater_or_equal::GreaterOrEqual, lower::Lower,
    lower_or_equal::LowerOrEqual,
};

fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    match (lhs, rhs) {
        (Type::Int, Type::Int) | (Type::Float, Type::Float) => true,
        (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
            if var_type.as_ref() == &Type::Int =>
        {
            true
        }
        (Type::Array(var_type), Type::Float) | (Type::Float, Type::Array(var_type))
            if var_type.as_ref() == &Type::Float =>
        {
            true
        }
        _ => false,
    }
}
