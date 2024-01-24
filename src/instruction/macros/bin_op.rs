#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! binOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::{
            local_variable::LocalVariables,
            traits::{BaseInstruction, BinOp, Recreate},
        };
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
        impl $T {
            pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction> {
                let lhs_type = lhs.return_type();
                let rhs_type = rhs.return_type();
                if Self::can_be_used(&lhs_type, &rhs_type) {
                    Self::create_from_instructions(lhs, rhs)
                } else {
                    Err(crate::Error::CannotDo2(lhs_type, $symbol, rhs_type))
                }
            }
        }

        impl Recreate for $T {
            fn recreate(
                &self,
                local_variables: &mut LocalVariables,
                interpreter: &Interpreter,
            ) -> Result<Instruction> {
                let lhs = self.lhs.recreate(local_variables, interpreter)?;
                let rhs = self.rhs.recreate(local_variables, interpreter)?;
                Self::create_from_instructions(lhs, rhs)
            }
        }
    };
}
