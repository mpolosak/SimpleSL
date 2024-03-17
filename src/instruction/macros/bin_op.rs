#[allow(clippy::crate_in_macro_def)]
macro_rules! binOp {
    ($T: ident, $symbol: literal) => {
        #[derive(Debug)]
        pub struct $T {
            pub lhs: crate::instruction::Instruction,
            pub rhs: crate::instruction::Instruction,
        }

        impl crate::instruction::traits::BaseInstruction for $T {}
        impl $T {
            pub fn create_op(
                lhs: crate::instruction::Instruction,
                rhs: crate::instruction::Instruction,
            ) -> Result<crate::instruction::Instruction, crate::errors::Error> {
                use crate::variable::ReturnType;
                let lhs_type = lhs.return_type();
                let rhs_type = rhs.return_type();
                use crate::instruction::traits::{self, CanBeUsed};
                if !Self::can_be_used(&lhs_type, &rhs_type) {
                    return Err(crate::errors::Error::CannotDo2(lhs_type, $symbol, rhs_type));
                }
                traits::ToResult::to_result(Self::create_from_instructions(lhs, rhs))
            }
        }
    };
    ($T: ident, $symbol: literal, cfi) => {
        binOp!($T, $symbol);
        impl $T {
            pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
                Self { lhs, rhs }.into()
            }
        }
    };
}
pub(crate) use binOp;
