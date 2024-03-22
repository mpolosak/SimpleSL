mod bitwise;
mod filter;
mod logic;
mod map;
mod math;
mod reduce;
mod shift;
use super::{array::Array, array_repeat::ArrayRepeat, local_variable::LocalVariables};
use crate::{
    instruction::{
        traits::{CanBeUsed, ToResult},
        Instruction,
    },
    parse::{unexpected, Rule},
    variable::{ReturnType, Variable},
    Error, Interpreter,
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
    pub fn create_op(lhs: Instruction, rhs: Instruction) -> Result<Instruction, Error> {
        let lhs_type = lhs.return_type();
        let rhs_type = rhs.return_type();
        if !Self::can_be_used(&lhs_type, &rhs_type) {
            return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
        }
        Self::create_from_instructions(lhs, rhs).to_result()
    }
}

#[duplicate_item(T; [Multiply]; [Subtract]; [Greater]; [GreaterOrEqual]; [Lower];
    [LowerOrEqual]; [BitwiseAnd]; [BitwiseOr]; [Xor]; [Equal])]
impl T {
    pub fn create_from_instructions(lhs: Instruction, rhs: Instruction) -> Instruction {
        match (lhs, rhs) {
            (Instruction::Variable(lhs), Instruction::Variable(rhs)) => Self::exec(lhs, rhs).into(),
            (Instruction::Array(array), rhs) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(|lhs| Self::create_from_instructions(lhs, rhs.clone()))
                    .collect();
                Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into()
            }
            (lhs, Instruction::Array(array)) => {
                let instructions = array
                    .instructions
                    .iter()
                    .cloned()
                    .map(|rhs| Self::create_from_instructions(lhs.clone(), rhs))
                    .collect();
                Array {
                    instructions,
                    var_type: array.var_type.clone(),
                }
                .into()
            }
            (Instruction::ArrayRepeat(array_repeat), rhs) => {
                let value = Self::create_from_instructions(array_repeat.value.clone(), rhs);
                ArrayRepeat {
                    value,
                    len: array_repeat.len.clone(),
                }
                .into()
            }
            (lhs, Instruction::ArrayRepeat(array_repeat)) => {
                let value = Self::create_from_instructions(lhs, array_repeat.value.clone());
                ArrayRepeat {
                    value,
                    len: array_repeat.len.clone(),
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

impl Instruction {
    pub fn create_infix(
        op: Pair<'_, Rule>,
        lhs: Self,
        rhs: Self,
        local_variables: &LocalVariables<'_>,
        interpreter: &Interpreter<'_>,
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
            Rule::reduce => Reduce::create_instruction(lhs, op, rhs, local_variables, interpreter),
            rule => unexpected(rule),
        }
    }
}
