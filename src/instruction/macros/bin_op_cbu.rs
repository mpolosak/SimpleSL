#[allow(clippy::crate_in_macro_def)]
macro_rules! binOpCBU {
    ($T: ident, $symbol: literal) => {
        crate::instruction::macros::binOp!($T, $symbol);
        impl crate::instruction::traits::CanBeUsed for $T {
            fn can_be_used(lhs: &crate::variable::Type, rhs: &crate::variable::Type) -> bool {
                crate::variable::Type::Tuple([lhs.clone(), rhs.clone()].into())
                    .matches(&ACCEPTED_TYPE)
            }
        }
    };
}
pub(crate) use binOpCBU;
