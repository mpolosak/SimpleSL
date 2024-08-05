use super::local_variable::LocalVariables;
use super::{Exec, ExecResult, InstructionWithStr, Recreate};
use crate::variable::{ReturnType, Type};
use crate::{self as simplesl, ExecError, Interpreter};
use crate::{instruction::Instruction, Error};
use duplicate::duplicate_item;
use lazy_static::lazy_static;
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::{unexpected, Rule};

#[derive(Debug)]
pub struct PrefixOperation {
    pub instruction: Instruction,
    pub op: PrefixOperator,
}

#[derive(Debug)]
pub enum PrefixOperator {
    BitwiseNot,
    Not,
    UnaryMinus,
}

impl Exec for PrefixOperation {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let variable = self.instruction.exec(interpreter)?;
        Ok(match self.op {
            PrefixOperator::BitwiseNot => bitwise_not::calc(variable),
            PrefixOperator::Not => not::calc(variable),
            PrefixOperator::UnaryMinus => unary_minus::calc(variable),
        })
    }
}

impl Recreate for PrefixOperation {
    fn recreate(&self, local_variables: &mut LocalVariables) -> Result<Instruction, ExecError> {
        let instruction = self.instruction.recreate(local_variables)?;
        Ok(match self.op {
            PrefixOperator::BitwiseNot => bitwise_not::create_from_instruction(instruction),
            PrefixOperator::Not => not::create_from_instruction(instruction),
            PrefixOperator::UnaryMinus => unary_minus::create_from_instruction(instruction),
        })
    }
}

impl ReturnType for PrefixOperation {
    fn return_type(&self) -> Type {
        self.instruction.return_type()
    }
}

impl From<PrefixOperation> for Instruction {
    fn from(value: PrefixOperation) -> Self {
        Self::PrefixOperation(value.into())
    }
}

impl InstructionWithStr {
    pub fn create_prefix(op: Pair<'_, Rule>, rhs: Self) -> Result<Self, Error> {
        let instruction = rhs.instruction;
        let instruction = match op.as_rule() {
            Rule::not => not::create_instruction(instruction),
            Rule::bitwise_not => bitwise_not::create_instruction(instruction),
            Rule::unary_minus => unary_minus::create_instruction(instruction),
            rule => unexpected!(rule),
        }?;
        let str = format!("{} {}", op.as_str(), rhs.str).into();
        Ok(Self { instruction, str })
    }
}

lazy_static! {
    pub static ref ACCEPTED_INT: Type = var_type!(int | [int]);
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = var_type!(int | float | [int | float]);
}

mod unary_minus {
    use crate as simplesl;
    use crate::variable::{Array, ReturnType, Type};
    use crate::{
        instruction::{
            prefix_op::{PrefixOperation, PrefixOperator},
            Instruction,
        },
        variable::Variable,
        Error,
    };
    use match_any::match_any;
    use simplesl_macros::var;
    use std::sync::Arc;

    use super::ACCEPTED_NUM;

    pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
        let return_type = instruction.return_type();
        if !can_be_used(&return_type) {
            return Err(Error::CannotDo(stringify!(symbol), return_type));
        }
        Ok(create_from_instruction(instruction))
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match_any! { instruction,
            Instruction::Variable(operand) => calc(operand).into(),
            Instruction::Array(array)
            | Instruction::ArrayRepeat(array) => Arc::unwrap_or_clone(array)
                .map(create_from_instruction)
                .into(),
            instruction => PrefixOperation {instruction,op:PrefixOperator::UnaryMinus }.into()
        }
    }

    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => var!(-num),
            Variable::Float(num) => var!(-num),
            Variable::Array(array) => {
                let elements = array.iter().cloned().map(calc).collect();
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

#[duplicate_item(t T op1 op2; [not] [Not] [num==0] [!]; [bitwise_not] [BitwiseNot] [!num] [~];)]
mod t {
    use std::sync::Arc;

    use crate::{
        instruction::{
            prefix_op::{PrefixOperation, PrefixOperator},
            Instruction,
        },
        variable::{Array, ReturnType, Type, Variable},
        Error,
    };
    use match_any::match_any;

    use super::ACCEPTED_INT;

    pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
        let return_type = instruction.return_type();
        if !can_be_used(&return_type) {
            return Err(Error::CannotDo(stringify!(symbol), return_type));
        }
        Ok(create_from_instruction(instruction))
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match_any! { instruction,
            Instruction::Variable(operand) => calc(operand).into(),
            Instruction::Array(array)
            | Instruction::ArrayRepeat(array) => Arc::unwrap_or_clone(array)
                .map(create_from_instruction)
                .into(),
            instruction => PrefixOperation {instruction, op: PrefixOperator::T } .into()
        }
    }
    pub fn calc(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => (op1).into(),
            Variable::Array(array) => {
                let elements = array.iter().cloned().map(calc).collect();
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
