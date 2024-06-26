use super::InstructionWithStr;
use crate as simplesl;
use crate::variable::{Array, ReturnType, Type, Variable};
use crate::{instruction::Instruction, Error};
use duplicate::duplicate_item;
use lazy_static::lazy_static;
use match_any::match_any;
use pest::iterators::Pair;
use simplesl_macros::{var, var_type};
use simplesl_parser::{unexpected, Rule};
use std::sync::Arc;

#[duplicate_item(T; [Not]; [BitwiseNot]; [UnaryMinus])]
#[derive(Debug)]
pub struct T {
    pub instruction: Instruction,
}

impl InstructionWithStr {
    pub fn create_prefix(op: Pair<'_, Rule>, rhs: Self) -> Result<Self, Error> {
        let instruction = rhs.instruction;
        let instruction = match op.as_rule() {
            Rule::not => Not::create_instruction(instruction),
            Rule::bitwise_not => BitwiseNot::create_instruction(instruction),
            Rule::unary_minus => UnaryMinus::create_instruction(instruction),
            rule => unexpected!(rule),
        }?;
        let str = format!("{} {}", op.as_str(), rhs.str).into();
        Ok(Self { instruction, str })
    }
}

#[duplicate_item(T symbol; [UnaryMinus] [-]; [Not] [!]; [BitwiseNot] [~])]
impl T {
    pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
        let return_type = instruction.return_type();
        if !Self::can_be_used(&return_type) {
            return Err(Error::CannotDo(stringify!(symbol), return_type));
        }
        Ok(Self::create_from_instruction(instruction))
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match_any! { instruction,
            Instruction::Variable(operand) => Self::calc(operand).into(),
            Instruction::Array(array)
            | Instruction::ArrayRepeat(array) => Arc::unwrap_or_clone(array)
                .map(Self::create_from_instruction)
                .into(),
            instruction => Self { instruction }.into()
        }
    }
}

lazy_static! {
    pub static ref ACCEPTED_INT: Type = var_type!(int | [int]);
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = var_type!(int | float | [int | float]);
}

impl UnaryMinus {
    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => var!(-num),
            Variable::Float(num) => var!(-num),
            Variable::Array(array) => {
                let elements = array.iter().cloned().map(Self::calc).collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            }
            operand => panic!("Tried to - {operand}"),
        }
    }

    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&ACCEPTED_NUM)
    }
}

#[duplicate_item(T op1 op2; [Not] [num==0] [!]; [BitwiseNot] [!num] [~];)]
impl T {
    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => (op1).into(),
            Variable::Array(array) => {
                let elements = array.iter().cloned().map(Self::calc).collect();
                let element_type = array.element_type().clone();
                Array {
                    element_type,
                    elements,
                }
                .into()
            }
            operand => panic!("Tried to {} {operand}", stringify!(op2)),
        }
    }
    fn can_be_used(var_type: &Type) -> bool {
        var_type.matches(&ACCEPTED_INT)
    }
}

#[cfg(test)]
mod tests {
    use crate::{self as simplesl, variable::Variable, Code, Error, Interpreter};
    use simplesl_macros::{var, var_type};

    #[test]
    fn prefix_ops() {
        assert_eq!(parse_and_exec("-5"), Ok(var!(-5)));
        assert_eq!(parse_and_exec("-7.5"), Ok(var!(-7.5)));
        assert_eq!(parse_and_exec("-[7.5, -4, 3]"), Ok(var!([-7.5, 4, -3])));
        assert_eq!(parse_and_exec("!5"), Ok(var!(0)));
        assert_eq!(parse_and_exec("!0"), Ok(var!(1)));
        assert_eq!(
            parse_and_exec("!7.5"),
            Err(Error::CannotDo("!", var_type!(float)))
        );
        assert_eq!(parse_and_exec("![7, -4, 0]"), Ok(var!([0, 0, 1])));
        assert_eq!(
            parse_and_exec("![7, -4.5, 0]"),
            Err(Error::CannotDo("!", var_type!([int | float])))
        );
        assert_eq!(parse_and_exec("~5"), Ok(Variable::Int(!5)));
        assert_eq!(parse_and_exec("~0"), Ok(Variable::Int(!0)));
        assert_eq!(
            parse_and_exec("~7.5"),
            Err(Error::CannotDo("~", var_type!(float)))
        );
        assert_eq!(
            parse_and_exec("~[7, -4, 0]"),
            Ok(Variable::from([
                Variable::Int(!7),
                Variable::Int(!(-4)),
                Variable::Int(!0)
            ]))
        );
        assert_eq!(
            parse_and_exec("~[7, -4.5, 0]"),
            Err(Error::CannotDo("~", var_type!([int | float])))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
