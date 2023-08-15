use super::traits::{BinOp, CanBeUsed};
use crate::variable::{GetReturnType, Type};
mod bitwise_and;
mod bitwise_not;
mod bitwise_or;
mod lshift;
mod rshift;
mod xor;
pub use {
    bitwise_and::BitwiseAnd, bitwise_not::BitwiseNot, bitwise_or::BitwiseOr, lshift::LShift,
    rshift::RShift, xor::Xor,
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
            (
                self.get_lhs().get_return_type(),
                self.get_rhs().get_return_type()
            ),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            Type::Array(Type::Int.into())
        } else {
            Type::Int
        }
    }
}
