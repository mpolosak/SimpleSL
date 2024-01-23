use super::{BinOp, CanBeUsed};
use crate::variable::{ReturnType, Type};
use lazy_static::lazy_static;
use std::str::FromStr;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = Type::from_str("(int, int|[int]) | ([int], int)").unwrap();
}
pub trait BinIntOp: BinOp {}

impl<T: BinIntOp> CanBeUsed for T {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        Type::Tuple([lhs.clone(), rhs.clone()].into()).matches(&ACCEPTED_TYPE)
    }
}

impl<T: BinIntOp> ReturnType for T {
    fn return_type(&self) -> Type {
        if matches!(
            (self.lhs().return_type(), self.rhs().return_type()),
            (Type::Array(_), _) | (_, Type::Array(_))
        ) {
            [Type::Int].into()
        } else {
            Type::Int
        }
    }
}
