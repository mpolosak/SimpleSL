mod bitwise;
mod filter;
mod logic;
mod map;
mod math;
mod reduce;
mod shift;
use std::sync::Arc;

use super::{array_repeat::ArrayRepeat, local_variable::LocalVariables, InstructionWithStr};
use crate::{
    instruction::{
        traits::{CanBeUsed, ToResult},
        Instruction,
    },
    parse::{unexpected, Rule},
    variable::{ReturnType, Variable},
    Error,
};
use duplicate::duplicate_item;
use pest::iterators::Pair;
pub use reduce::*;

#[duplicate_item(T; [BitwiseAnd]; [BitwiseOr]; [Xor]; [Equal]; [Filter]; [Map]; [And]; [Or];
    [Add]; [Subtract]; [Multiply]; [Divide]; [Modulo]; [Pow]; [Greater]; [GreaterOrEqual];
    [Lower]; [LowerOrEqual]; [LShift]; [RShift]
)]
#[derive(Debug)]
pub struct T {
    pub lhs: Instruction,
    pub rhs: Instruction,
}

#[duplicate_item(T op; [BitwiseAnd] [&]; [BitwiseOr] [|]; [Xor] [^]; [Equal] [==]; [Filter] [?];
    [Map] [@]; [And] [&&]; [Or] [||]; [Add] [+]; [Subtract] [-]; [Multiply] [*]; [Divide] [/];
    [Modulo] [%]; [Pow] [**]; [Greater] [>]; [GreaterOrEqual] [>=]; [Lower] [<];
    [LowerOrEqual] [<=]; [LShift] [<<]; [RShift] [>>]
)]
impl T {
    pub fn create_op(
        lhs: InstructionWithStr,
        rhs: InstructionWithStr,
    ) -> Result<InstructionWithStr, Error> {
        let str = format!("{} {} {}", lhs.str, stringify!(op), rhs.str).into();
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
        }
        let instruction: Result<Instruction, Error> =
            Self::create_from_instructions(lhs.instruction, rhs.instruction).to_result();
        Ok(InstructionWithStr {
            instruction: instruction?,
            str,
        })
    }
}

#[duplicate_item(T; [Multiply]; [Subtract]; [Greater]; [GreaterOrEqual]; [Lower];
    [LowerOrEqual]; [BitwiseAnd]; [BitwiseOr]; [Xor]; [Equal])]
impl T {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (Instruction::Array(array), rhs) => Arc::unwrap_or_clone(array)
                .map(|lhs| Self::create_from_instructions(lhs, rhs.clone()))
                .into(),
            (lhs, Instruction::Array(array)) => Arc::unwrap_or_clone(array)
                .map(|rhs| Self::create_from_instructions(lhs.clone(), rhs))
                .into(),
            (Instruction::ArrayRepeat(array_repeat), rhs) => {
                let array_repeat = Arc::unwrap_or_clone(array_repeat);
                let value = array_repeat
                    .value
                    .map(|lhs| Self::create_from_instructions(lhs, rhs));
                ArrayRepeat {
                    value,
                    len: array_repeat.len,
                }
                .into()
            }
            (lhs, Instruction::ArrayRepeat(array_repeat)) => {
                let array_repeat = Arc::unwrap_or_clone(array_repeat);
                let value = array_repeat
                    .value
                    .map(|rhs| Self::create_from_instructions(lhs, rhs));
                ArrayRepeat {
                    value,
                    len: array_repeat.len,
                }
                .into()
            }
            (lhs, rhs) => Self { lhs, rhs }.into(),
        }
    }
}

#[duplicate_item(T; [Filter]; [Map])]
impl T {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        Self { lhs, rhs }.into()
    }
}

impl Equal {
    pub fn exec(lhs: Variable, rhs: Variable) -> Variable {
        (lhs == rhs).into()
    }
}

impl InstructionWithStr {
    pub fn create_infix(
        op: Pair<'_, Rule>,
        lhs: Self,
        rhs: Self,
        local_variables: &LocalVariables<'_>,
    ) -> Result<Self, Error> {
        match op.as_rule() {
            Rule::pow => Pow::create_op(lhs, rhs),
            Rule::multiply => Multiply::create_op(lhs, rhs),
            Rule::add => Add::create_op(lhs, rhs),
            Rule::subtract => Subtract::create_op(lhs, rhs),
            Rule::divide => Divide::create_op(lhs, rhs),
            Rule::modulo => Modulo::create_op(lhs, rhs),
            Rule::equal => Equal::create_op(lhs, rhs),
            Rule::lower => Lower::create_op(lhs, rhs),
            Rule::lower_equal => LowerOrEqual::create_op(lhs, rhs),
            Rule::greater => Greater::create_op(lhs, rhs),
            Rule::greater_equal => GreaterOrEqual::create_op(lhs, rhs),
            Rule::map => Map::create_op(lhs, rhs),
            Rule::filter => Filter::create_op(lhs, rhs),
            Rule::bitwise_and => BitwiseAnd::create_op(lhs, rhs),
            Rule::bitwise_or => BitwiseOr::create_op(lhs, rhs),
            Rule::xor => Xor::create_op(lhs, rhs),
            Rule::rshift => RShift::create_op(lhs, rhs),
            Rule::lshift => LShift::create_op(lhs, rhs),
            Rule::and => And::create_op(lhs, rhs),
            Rule::or => Or::create_op(lhs, rhs),
            Rule::reduce => {
                let str = format!("{} {} {}", lhs.str, op.as_str(), rhs.str).into();
                let instruction = Reduce::create_instruction(lhs, op, rhs, local_variables)?;
                Ok(InstructionWithStr { instruction, str })
            }
            rule => unexpected(rule),
        }
    }
}
