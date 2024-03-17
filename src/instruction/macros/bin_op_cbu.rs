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

        impl crate::instruction::Exec for $T {
            fn exec(&self, interpreter: &mut crate::Interpreter) -> crate::instruction::ExecResult {
                let lhs = self.lhs.exec(interpreter)?;
                let rhs = self.rhs.exec(interpreter)?;
                crate::instruction::traits::ToResult::to_result(Self::exec(lhs, rhs))
            }
        }
    };
}
pub(crate) use binOpCBU;
