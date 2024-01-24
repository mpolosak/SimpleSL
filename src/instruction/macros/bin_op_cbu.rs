#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! binOpCBU {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::traits::CanBeUsed;
        crate::binOp!($T, $symbol);
        impl CanBeUsed for $T {
            fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
                Type::Tuple([lhs.clone(), rhs.clone()].into()).matches(&ACCEPTED_TYPE)
            }
        }

        impl Exec for $T {
            fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
                let lhs = self.lhs.exec(interpreter)?;
                let rhs = self.rhs.exec(interpreter)?;
                Self::exec(lhs, rhs)
            }
        }
    };
}
