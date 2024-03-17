pub mod bin_op;
pub mod bin_op_cbu;
use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;
pub(crate) use {bin_op::binOp, bin_op_cbu::binOpCBU};

lazy_static! {
    pub static ref ACCEPTED_INT_TYPE: Type =
        Type::from_str("(int, int|[int]) | ([int], int)").unwrap();
}

lazy_static! {
    pub static ref ACCEPTED_NUM_TYPE: Type = Type::from_str(
        "(int|[int], int) | (int, [int]) | (float|[float], float) | (float, [float]) "
    )
    .unwrap();
}
