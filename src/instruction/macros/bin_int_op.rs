use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;
lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = Type::from_str("(int, int|[int]) | ([int], int)").unwrap();
}

#[allow(clippy::crate_in_macro_def)]
macro_rules! binIntOp {
    ($T: ident, $symbol: literal) => {
        crate::instruction::macros::binOpCBU!($T, $symbol);

        impl crate::variable::ReturnType for $T {
            fn return_type(&self) -> crate::variable::Type {
                let lhs = self.lhs.return_type();
                let rhs = self.rhs.return_type();
                crate::instruction::ord::return_type(lhs, rhs)
            }
        }
    };
}

pub(crate) use binIntOp;
