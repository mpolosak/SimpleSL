use super::InstructionWithStr;
use crate as simplesl;
use crate::variable::Type;
use crate::Error;
use lazy_static::lazy_static;
use pest::iterators::Pair;
use simplesl_macros::var_type;
use simplesl_parser::{unexpected, Rule};

impl InstructionWithStr {
    pub fn create_prefix(op: Pair<'_, Rule>, rhs: Self) -> Result<Self, Error> {
        let instruction = rhs.instruction;
        let instruction = match op.as_rule() {
            Rule::not => not::create_instruction(instruction),
            Rule::unary_minus => unary_minus::create_instruction(instruction),
            Rule::indirection => indirection::create_instruction(instruction),
            rule => unexpected!(rule),
        }?;
        let str = format!("{} {}", op.as_str(), rhs.str).into();
        Ok(Self { instruction, str })
    }
}

lazy_static! {
    pub static ref ACCEPTED_NUM: Type = var_type!(int | float | [int | float]);
}

pub mod unary_minus {
    use crate as simplesl;
    use crate::instruction::unary_operation::{UnaryOperation, UnaryOperator};
    use crate::variable::{Array, ReturnType};
    use crate::{instruction::Instruction, variable::Variable, Error};
    use match_any::match_any;
    use simplesl_macros::var;
    use std::sync::Arc;

    use super::ACCEPTED_NUM;

    pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
        let return_type = instruction.return_type();
        if !return_type.matches(&ACCEPTED_NUM) {
            return Err(Error::CannotDo("-", return_type));
        }
        Ok(UnaryOperation {
            instruction,
            op: UnaryOperator::UnaryMinus,
        }
        .into())
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match_any! { instruction,
            Instruction::Variable(operand) => exec(operand).into(),
            Instruction::Array(array)
            | Instruction::ArrayRepeat(array) => Arc::unwrap_or_clone(array)
                .map(create_from_instruction)
                .into(),
            instruction => UnaryOperation {instruction,op:UnaryOperator::UnaryMinus }.into()
        }
    }

    pub fn exec(variable: Variable) -> Variable {
        match variable {
            Variable::Int(num) => var!(-num),
            Variable::Float(num) => var!(-num),
            Variable::Array(array) => {
                let elements = array.iter().cloned().map(exec).collect();
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
}

pub mod not {
    use crate as simplesl;
    use crate::{
        instruction::{
            unary_operation::{UnaryOperation, UnaryOperator},
            Instruction,
        },
        variable::{Array, ReturnType, Type, Variable},
        Error,
    };
    use lazy_static::lazy_static;
    use match_any::match_any;
    use simplesl_macros::var_type;
    use std::sync::Arc;

    lazy_static! {
        pub static ref ACCEPTED_INT: Type = var_type!(int | bool | [int | bool]);
    }

    pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
        let return_type = instruction.return_type();
        if !return_type.matches(&ACCEPTED_INT) {
            return Err(Error::CannotDo("!", return_type));
        }
        Ok(UnaryOperation {
            instruction,
            op: UnaryOperator::Not,
        }
        .into())
    }

    pub fn create_from_instruction(instruction: Instruction) -> Instruction {
        match_any! { instruction,
            Instruction::Variable(operand) => exec(operand).into(),
            Instruction::Array(array)
            | Instruction::ArrayRepeat(array) => Arc::unwrap_or_clone(array)
                .map(create_from_instruction)
                .into(),
            instruction => UnaryOperation {instruction, op: UnaryOperator::Not } .into()
        }
    }
    pub fn exec(variable: Variable) -> Variable {
        match variable {
            Variable::Bool(var) => (!var).into(),
            Variable::Int(num) => (!num).into(),
            Variable::Array(array) => {
                let elements = array.iter().cloned().map(exec).collect();
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
}

pub mod indirection {
    use crate::{
        instruction::{
            unary_operation::{UnaryOperation, UnaryOperator},
            Instruction,
        },
        variable::{ReturnType, Type, Variable},
        Error,
    };

    pub fn create_instruction(instruction: Instruction) -> Result<Instruction, Error> {
        let return_type = instruction.return_type();
        if !return_type.is_mut() {
            return Err(Error::CannotDo("*", return_type));
        }
        Ok(UnaryOperation {
            instruction,
            op: UnaryOperator::Indirection,
        }
        .into())
    }

    pub fn exec(var: Variable) -> Variable {
        let var = var.into_mut().unwrap();
        let var = var.variable.read().unwrap().clone();
        var
    }

    pub fn return_type(var_type: Type) -> Type {
        var_type.mut_element_type().unwrap()
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
        assert_eq!(parse_and_exec("!5"), Ok(var!(-6)));
        assert_eq!(parse_and_exec("!0"), Ok(var!(-1)));
        assert_eq!(
            parse_and_exec("!7.5"),
            Err(Error::CannotDo("!", var_type!(float)))
        );
        assert_eq!(parse_and_exec("![7, -4, 0]"), Ok(var!([-8, 3, -1])));
        assert_eq!(parse_and_exec("!true"), Ok(var!(false)));
        assert_eq!(parse_and_exec("!false"), Ok(var!(true)));
        assert_eq!(parse_and_exec("![7, true, 0]"), Ok(var!([-8, false, -1])));
        assert_eq!(
            parse_and_exec("![7, -4.5, 0]"),
            Err(Error::CannotDo("!", var_type!([int | float])))
        );
        assert_eq!(
            parse_and_exec("![7, -4.5, 0]"),
            Err(Error::CannotDo("!", var_type!([int | float])))
        );
    }

    fn parse_and_exec(script: &str) -> Result<Variable, crate::Error> {
        Code::parse(&Interpreter::without_stdlib(), script)
            .and_then(|code| code.exec().map_err(Error::from))
    }
}
