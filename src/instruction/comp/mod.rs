use super::Instruction;
use crate::variable::{GetReturnType, Type};
mod equal;
mod greater;
mod greater_or_equal;
pub use {equal::Equal, greater::Greater, greater_or_equal::GreaterOrEqual};

fn can_be_used(lhs: &Instruction, rhs: &Instruction) -> bool {
    match (
        lhs.get_return_type().as_ref(),
        rhs.get_return_type().as_ref(),
    ) {
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
