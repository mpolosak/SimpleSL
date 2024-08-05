use crate as simplesl;
use crate::variable::Type;
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_INT_TYPE: Type = var_type!((int, int | [int]) | ([int], int));
}

pub fn can_be_used_int(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_INT_TYPE)
}

lazy_static! {
    pub static ref ACCEPTED_NUM_TYPE: Type =
        var_type!((int | [int], int) | (int, [int]) | (float | [float], float) | (float, [float]));
}

pub fn can_be_used_num(lhs: Type, rhs: Type) -> bool {
    var_type!((lhs, rhs)).matches(&ACCEPTED_NUM_TYPE)
}
