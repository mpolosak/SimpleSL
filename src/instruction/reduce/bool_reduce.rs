use crate::function::Function;
use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::variable::ReturnType;
use crate::{self as simplesl, Code, Error, ExecError};
use crate::{
    variable::{Type, Variable},
    Interpreter,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, bool));
}

lazy_static! {
    static ref ALL: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, bool)) -> () -> bool {
            loop {
                (con, value) := iter();
                if !con break;
                if !value return false;
            }
            return true;
        }"
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

lazy_static! {
    static ref ANY: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, bool)) -> () -> bool {
            loop {
                (con, value) := iter();
                if !con break;
                if value return true;
            }
            return false;
        }"
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

pub fn create(array: InstructionWithStr, op: UnaryOperator) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&ACCEPTED_TYPE) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: array.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op,
    }
    .into())
}

pub fn all(var: Variable) -> Result<Variable, ExecError> {
    ALL.exec_with_args(&[var])
}

pub fn any(var: Variable) -> Result<Variable, ExecError> {
    ANY.exec_with_args(&[var])
}
