use crate::function::Function;
use crate::instruction::unary_operation::UnaryOperation;
use crate::variable::{Type, Variable};
use crate::{self as simplesl, Code, ExecError, Interpreter};
use crate::{
    instruction::{Instruction, InstructionWithStr},
    unary_operator::UnaryOperator,
    variable::ReturnType,
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int));
}

lazy_static! {
    pub static ref AND: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $!0 (acc: int, curr: int) -> int {
                return acc & curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

lazy_static! {
    pub static ref OR: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $0 (acc: int, curr: int) -> int {
                return acc | curr;
            }
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

pub fn and(var: Variable) -> Result<Variable, ExecError> {
    AND.exec_with_args(&[var])
}

pub fn or(var: Variable) -> Result<Variable, ExecError> {
    OR.exec_with_args(&[var])
}
