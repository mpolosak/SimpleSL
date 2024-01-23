use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;
lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = Type::from_str("(int, int|[int]) | ([int], int)").unwrap();
}

#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! binNumOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::{
            traits::{BaseInstruction, BinOp, CanBeUsed},
            Exec,
        };
        use crate::{
            variable::{ReturnType, Type},
            Interpreter,
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

        impl CanBeUsed for $T {
            fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
                Type::Tuple([lhs.clone(), rhs.clone()].into())
                    .matches(&crate::instruction::macros::bin_num_op::ACCEPTED_TYPE)
            }
        }

        impl ReturnType for $T {
            fn return_type(&self) -> Type {
                if matches!(
                    (self.lhs().return_type(), self.rhs().return_type()),
                    (Type::Array(_), _) | (_, Type::Array(_))
                ) {
                    [Type::Int].into()
                } else {
                    Type::Int
                }
            }
        }

        impl Exec for $T {
            fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
                let lhs = self.lhs().exec(interpreter)?;
                let rhs = self.rhs().exec(interpreter)?;
                Self::exec(lhs, rhs)
            }
        }

        impl BaseInstruction for $T {}
    };
}
