use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;

lazy_static! {
    pub static ref ACCEPTED_INT: Type = Type::from_str("int|[int]").unwrap();
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = Type::from_str("int|float|[int|float]").unwrap();
}

#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! prefixOp {
    ($T: ident, $symbol: literal) => {
        use crate::instruction::{
            local_variable::LocalVariables,
            traits::{Exec, Recreate},
            Instruction,
        };
        use crate::{
            variable::{ReturnType, Type, Variable},
            Error, Interpreter, Result,
        };
        #[derive(Debug)]
        pub struct $T {
            pub instruction: Instruction,
        }
        #[allow(dead_code)]
        impl $T {
            pub fn create_instruction(instruction: Instruction) -> Result<Instruction> {
                let return_type = instruction.return_type();
                if !Self::can_be_used(&return_type) {
                    return Err(Error::CannotDo($symbol, return_type));
                }
                Ok(Self::create_from_instruction(instruction))
            }

            fn create_from_instruction(instruction: Instruction) -> Instruction {
                match instruction {
                    Instruction::Variable(operand) => Self::calc(operand).into(),
                    instruction => Self { instruction }.into(),
                }
            }

            fn can_be_used(var_type: &Type) -> bool {
                var_type.matches(&ACCEPTED_TYPE)
            }
        }

        impl ReturnType for $T {
            fn return_type(&self) -> Type {
                self.instruction.return_type()
            }
        }

        impl Exec for $T {
            fn exec(&self, interpreter: &mut Interpreter) -> Result<Variable> {
                let result = self.instruction.exec(interpreter)?;
                Ok(Self::calc(result))
            }
        }

        impl Recreate for $T {
            fn recreate(
                &self,
                local_variables: &mut LocalVariables,
                interpreter: &Interpreter,
            ) -> Result<Instruction> {
                let instruction = self.instruction.recreate(local_variables, interpreter)?;
                Ok(Self::create_from_instruction(instruction))
            }
        }

        impl crate::instruction::BaseInstruction for $T {}
    };
    ($T: ident, $symbol: literal, int, $calc: expr) => {
        use crate::instruction::macros::prefix_op::ACCEPTED_INT as ACCEPTED_TYPE;
        prefixOp!($T, $symbol);
        #[allow(clippy::redundant_closure_call)]
        impl $T {
            fn calc(variable: Variable) -> Variable {
                match variable {
                    Variable::Int(num) => $calc(num).into(),
                    Variable::Array(array) => array.iter().cloned().map(Self::calc).collect(),
                    operand => panic!("Tried to {} {operand}", $symbol),
                }
            }
        }
    };
    ($T: ident, $symbol: literal, num, $calc: expr) => {
        use crate::instruction::macros::prefix_op::ACCEPTED_INT as ACCEPTED_TYPE;
        prefixOp!($T, $symbol);
        impl $T {
            fn calc(variable: Variable) -> Variable {
                match variable {
                    Variable::Int(num) => $calc(num).into(),
                    Variable::Float(num) => $calc(num).into(),
                    Variable::Array(array) => array.iter().cloned().map(Self::calc).collect(),
                    operand => panic!("Tried to {} {operand}", $symbol),
                }
            }
        }
    };
}
