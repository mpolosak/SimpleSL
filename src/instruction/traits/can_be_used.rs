use crate as simplesl;
use crate::{instruction::bin_op::*, variable::Type};
use duplicate::duplicate_item;
use lazy_static::lazy_static;
use simplesl_macros::var_type;

pub trait CanBeUsed {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool;
}

lazy_static! {
    pub static ref ACCEPTED_INT_TYPE: Type = var_type!((int, int | [int]) | ([int], int));
}

#[duplicate_item(T; [Modulo]; [And]; [Or]; [BitwiseAnd]; [BitwiseOr]; [Xor]; [LShift]; [RShift])]
impl CanBeUsed for T {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let lhs = lhs.clone();
        let rhs = rhs.clone();
        var_type!((lhs, rhs)).matches(&ACCEPTED_INT_TYPE)
    }
}

lazy_static! {
    pub static ref ACCEPTED_NUM_TYPE: Type =
        var_type!((int | [int], int) | (int, [int]) | (float | [float], float) | (float, [float]));
}

#[duplicate_item(T; [Greater]; [GreaterOrEqual]; [Lower]; [LowerOrEqual]; [Multiply]; [Divide]; [Subtract]; [Pow])]
impl CanBeUsed for T {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let lhs = lhs.clone();
        let rhs = rhs.clone();
        var_type!((lhs, rhs)).matches(&ACCEPTED_NUM_TYPE)
    }
}

#[duplicate_item(T; [Equal]; [NotEqual])]
impl CanBeUsed for T {
    fn can_be_used(_: &Type, _: &Type) -> bool {
        true
    }
}
