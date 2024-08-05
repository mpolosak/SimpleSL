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

pub fn can_be_used_int(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_INT_TYPE)
}

#[duplicate_item(T; [And]; [Or];)]
impl CanBeUsed for T {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        let lhs = lhs.clone();
        let rhs = rhs.clone();
        can_be_used_int(lhs, rhs)
    }
}

lazy_static! {
    pub static ref ACCEPTED_NUM_TYPE: Type =
        var_type!((int | [int], int) | (int, [int]) | (float | [float], float) | (float, [float]));
}

pub fn can_be_used_num(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_NUM_TYPE)
}
