use crate::{instruction::bin_op::*, variable::Type};
use duplicate::duplicate_item;
use lazy_static::lazy_static;
use std::str::FromStr;

pub trait CanBeUsed {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool;
}

lazy_static! {
    pub static ref ACCEPTED_INT_TYPE: Type =
        Type::from_str("(int, int|[int]) | ([int], int)").unwrap();
}

#[duplicate_item(T; [Modulo]; [And]; [Or]; [BitwiseAnd]; [BitwiseOr]; [Xor]; [LShift]; [RShift])]
impl CanBeUsed for T {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        Type::Tuple([lhs.clone(), rhs.clone()].into()).matches(&ACCEPTED_INT_TYPE)
    }
}

lazy_static! {
    pub static ref ACCEPTED_NUM_TYPE: Type = Type::from_str(
        "(int|[int], int) | (int, [int]) | (float|[float], float) | (float, [float]) "
    )
    .unwrap();
}

#[duplicate_item(T; [Greater]; [GreaterOrEqual]; [Lower]; [LowerOrEqual]; [Multiply]; [Divide]; [Subtract]; [Pow])]
impl CanBeUsed for T {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        Type::Tuple([lhs.clone(), rhs.clone()].into()).matches(&ACCEPTED_NUM_TYPE)
    }
}

lazy_static! {
    static ref ACCEPTED_TYPE: Type = Type::from_str(
        "(int|[int], int|[int]) | (float|[float], float|[float]) | (string|[string], string|[string]) | ([any], [any])"
    )
    .unwrap();
}

impl CanBeUsed for Add {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        Type::Tuple([lhs.clone(), rhs.clone()].into()).matches(&ACCEPTED_TYPE)
    }
}
