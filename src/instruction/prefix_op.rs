use crate::variable::{Type, Variable};
use crate::{
    instruction::Instruction,
    parse::{unexpected, Rule},
    Error,
};
use duplicate::duplicate_item;
use lazy_static::lazy_static;
use pest::iterators::Pair;
use std::str::FromStr;

use super::array::Array;
use super::array_repeat::ArrayRepeat;

#[duplicate_item(T; [Not]; [BitwiseNot]; [UnaryMinus])]
#[derive(Debug)]
pub struct T {
    pub instruction: Instruction,
}

impl Instruction {
    pub fn create_prefix(op: Pair<'_, Rule>, rhs: Self) -> Result<Self, Error> {
        match op.as_rule() {
            Rule::not => Not::create_instruction(rhs),
            Rule::bitwise_not => BitwiseNot::create_instruction(rhs),
            Rule::unary_minus => UnaryMinus::create_instruction(rhs),
            rule => unexpected(rule),
        }
    }
}

#[duplicate_item(T symbol; [UnaryMinus] [-]; [Not] [!]; [BitwiseNot] [~])]
impl T {
    pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
        use crate::variable::ReturnType;
        let return_type = instruction.return_type();
        if !Self::can_be_used(&return_type) {
            return Err(Error::CannotDo(stringify!(symbol), return_type));
        }
        Ok(Self::create_from_instruction(instruction))
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match instruction {
            Instruction::Variable(operand) => Self::calc(operand).into(),
            Instruction::Array(array) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(Self::create_from_instruction)
                    .collect();
                Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into()
            }
            Instruction::ArrayRepeat(array_repeat) => {
                let value = Self::create_from_instruction(array_repeat.value.clone());
                ArrayRepeat {
                    value,
                    len: array_repeat.len.clone(),
                }
                .into()
            }
            instruction => Self { instruction }.into(),
        }
    }
}

lazy_static! {
    pub static ref ACCEPTED_INT: Type = Type::from_str("int|[int]").unwrap();
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = Type::from_str("int|float|[int|float]").unwrap();
}

impl UnaryMinus {
    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => (-num).into(),
            Variable::Float(num) => (-num).into(),
            Variable::Array(array) => array.iter().cloned().map(Self::calc).collect(),
            operand => panic!("Tried to - {operand}"),
        }
    }

    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&ACCEPTED_INT)
    }
}

#[duplicate_item(T op1 op2; [Not] [num==0] [!]; [BitwiseNot] [!num] [~];)]
impl T {
    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => (op1).into(),
            Variable::Array(array) => array.iter().cloned().map(Self::calc).collect(),
            operand => panic!("Tried to {} {operand}", stringify!(op2)),
        }
    }
    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&ACCEPTED_INT)
    }
}
