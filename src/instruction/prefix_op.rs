mod r#macro;
use crate::Error;
use crate::{
    instruction::Instruction,
    parse::{unexpected, Rule},
};
use pest::iterators::Pair;
use r#macro::{prefixOp, ACCEPTED_INT, ACCEPTED_NUM};
use std::ops::Neg;

prefixOp!(UnaryMinus, "-", num, Neg::neg);
prefixOp!(Not, "!", int, |num| i64::from(num == 0));
prefixOp!(BitwiseNot, "~", int, |num: i64| !num);

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
