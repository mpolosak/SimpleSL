mod r#macro;
use crate::variable::Variable;
use crate::Error;
use crate::{
    instruction::Instruction,
    parse::{unexpected, Rule},
};
use duplicate::duplicate_item;
use pest::iterators::Pair;
use r#macro::{prefixOp, ACCEPTED_INT, ACCEPTED_NUM};

prefixOp!(UnaryMinus, "-", ACCEPTED_NUM);
prefixOp!(Not, "!", ACCEPTED_INT);
prefixOp!(BitwiseNot, "~", ACCEPTED_INT);

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

impl UnaryMinus {
    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => (-num).into(),
            Variable::Float(num) => (-num).into(),
            Variable::Array(array) => array.iter().cloned().map(Self::calc).collect(),
            operand => panic!("Tried to - {operand}"),
        }
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
}
