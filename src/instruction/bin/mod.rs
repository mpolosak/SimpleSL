use super::Instruction;
use crate::variable::{GetReturnType, Type};
mod bin_and;
mod bin_not;
mod bin_or;
mod lshift;
mod rshift;
mod xor;
pub use {
    bin_and::BinAnd, bin_not::BinNot, bin_or::BinOr, lshift::LShift, rshift::RShift, xor::Xor,
};

fn can_be_used(lhs: &Instruction, rhs: &Instruction) -> bool {
    match (lhs.get_return_type(), rhs.get_return_type()) {
        (Type::Int, Type::Int) => true,
        (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
            if var_type == Type::Int.into() =>
        {
            true
        }
        _ => false,
    }
}
