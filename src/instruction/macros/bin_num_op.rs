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
#[macro_export]
macro_rules! binNumOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::macros::bin_num_op::ACCEPTED_TYPE;
        use crate::instruction::Exec;
        use crate::{
            variable::{ReturnType, Type},
            Interpreter,
        };
        crate::binOp!($T, $symbol);

        impl ReturnType for $T {
            fn return_type(&self) -> Type {
                if matches!(
                    (self.lhs.return_type(), self.rhs.return_type()),
                    (Type::Array(_), _) | (_, Type::Array(_))
                ) {
                    [Type::Int].into()
                } else {
                    Type::Int
                }
            }
        }
    };
}
