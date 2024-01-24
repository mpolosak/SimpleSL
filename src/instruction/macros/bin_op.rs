#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! binOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::traits::{BaseInstruction, BinOp, CanBeUsed};
        #[derive(Debug)]
        pub struct $T {
            lhs: Instruction,
            rhs: Instruction,
        }

        impl BinOp for $T {
            const SYMBOL: &'static str = $symbol;

            fn lhs(&self) -> &Instruction {
                &self.lhs
            }

            fn rhs(&self) -> &Instruction {
                &self.rhs
            }

            fn construct(lhs: Instruction, rhs: Instruction) -> Self {
                Self { lhs, rhs }
            }
        }
        impl BaseInstruction for $T {}

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
