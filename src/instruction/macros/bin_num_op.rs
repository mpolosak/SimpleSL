use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;
lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = Type::from_str(
        "(int|[int], int) | (int, [int]) | (float|[float], float) | (float, [float]) "
    )
    .unwrap();
}

#[allow(clippy::crate_in_macro_def)]
macro_rules! binNumOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::macros::bin_num_op::ACCEPTED_TYPE;
        crate::instruction::macros::binOpCBU!($T, $symbol);
    };
}

pub(crate) use binNumOp;
