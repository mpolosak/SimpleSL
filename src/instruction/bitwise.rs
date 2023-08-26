use super::traits::{BinOp, CanBeUsed};
use crate::variable::{GetReturnType, Type};
mod and;
mod lshift;
mod not;
mod or;
mod rshift;
mod xor;
pub use {
    and::BitwiseAnd, lshift::LShift, not::BitwiseNot, or::BitwiseOr, rshift::RShift, xor::Xor,
};

trait BitwiseBinOp: BinOp {}

impl<T: BitwiseBinOp> CanBeUsed for T {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        match (lhs, rhs) {
            (Type::Int, Type::Int) => true,
            (Type::Array(var_type), Type::Int) | (Type::Int, Type::Array(var_type))
                if var_type.as_ref() == &Type::Int =>
            {
                true
            }
            _ => false,
        }
    }
}

impl<T: BitwiseBinOp> GetReturnType for T {
    fn get_return_type(&self) -> Type {
        if matches!(
            (self.lhs().get_return_type(), self.rhs().get_return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}
