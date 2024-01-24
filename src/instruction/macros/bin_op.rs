#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! binOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::traits::{BaseInstruction, BinOp};
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
    };
}
