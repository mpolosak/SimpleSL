use crate::variable::Type;
use lazy_static::lazy_static;
use std::str::FromStr;

lazy_static! {
    pub static ref ACCEPTED_INT: Type = Type::from_str("int|[int]").unwrap();
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = Type::from_str("int|float|[int|float]").unwrap();
}

macro_rules! prefixOp {
    ($T: ident, $symbol: literal, $accepted: ident) => {
        #[derive(Debug)]
        pub struct $T {
            pub instruction: Instruction,
        }
        #[allow(dead_code)]
        impl $T {
            pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
                use crate::variable::ReturnType;
                let return_type = instruction.return_type();
                if !Self::can_be_used(&return_type) {
                    return Err(Error::CannotDo($symbol, return_type));
                }
                Ok(Self::create_from_instruction(instruction))
            }

            pub fn create_from_instruction(instruction: Instruction) -> Instruction {
                match instruction {
                    Instruction::Variable(operand) => Self::calc(operand).into(),
                    instruction => Self { instruction }.into(),
                }
            }

            fn can_be_used(var_type: &crate::variable::Type) -> bool {
                var_type.matches(&$accepted)
            }
        }

        impl crate::variable::ReturnType for $T {
            fn return_type(&self) -> crate::variable::Type {
                self.instruction.return_type()
            }
        }

        impl crate::instruction::BaseInstruction for $T {}
    };
    ($T: ident, $symbol: literal, int, $calc: expr) => {
        prefixOp!($T, $symbol, ACCEPTED_INT);
        #[allow(clippy::redundant_closure_call)]
        impl $T {
            pub fn calc(variable: crate::variable::Variable) -> crate::variable::Variable {
                use crate::variable::Variable;
                match variable {
                    Variable::Int(num) => $calc(num).into(),
                    Variable::Array(array) => array.iter().cloned().map(Self::calc).collect(),
                    operand => panic!("Tried to {} {operand}", $symbol),
                }
            }
        }
    };
    ($T: ident, $symbol: literal, num, $calc: expr) => {
        prefixOp!($T, $symbol, ACCEPTED_NUM);
        impl $T {
            pub fn calc(variable: crate::variable::Variable) -> crate::variable::Variable {
                use crate::variable::Variable;
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

pub(crate) use prefixOp;
