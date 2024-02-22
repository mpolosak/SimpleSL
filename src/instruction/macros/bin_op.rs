#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! binOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::{
            local_variable::LocalVariables,
            traits::{BaseInstruction, Recreate},
        };
        use crate::{Error, ExecError};
        #[derive(Debug)]
        pub struct $T {
            lhs: Instruction,
            rhs: Instruction,
        }

        impl BaseInstruction for $T {}
        impl $T {
            pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
                let lhs_type = lhs.return_type();
                let rhs_type = rhs.return_type();
                if !Self::can_be_used(&lhs_type, &rhs_type) {
                    return Err(Error::CannotDo2(lhs_type, $symbol, rhs_type));
                }
                Ok(Self::create_from_instructions(lhs, rhs)?)
            }
        }

        impl Recreate for $T {
            fn recreate(
                &self,
                local_variables: &mut LocalVariables,
                interpreter: &Interpreter,
            ) -> Result<Instruction, ExecError> {
                let lhs = self.lhs.recreate(local_variables, interpreter)?;
                let rhs = self.rhs.recreate(local_variables, interpreter)?;
                Self::create_from_instructions(lhs, rhs)
            }
        }
    };
}
