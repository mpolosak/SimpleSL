use std::sync::Arc;

use crate::function::Function;
use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::ExecResult;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::variable::{Type, Typed};
use crate::{self as simplesl, Code, Interpreter};
use crate::{
    variable::{ReturnType, Variable},
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type =
        var_type!(() -> (bool, int) | () -> (bool, float) | () -> (bool, string));
}

lazy_static! {
    pub static ref INT_SUM: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $0 (acc: int, curr: int) -> int {
                return acc + curr;
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
    pub static ref FLOAT_SUM: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, float)) -> float {
            return iter $0.0 (acc: float, curr: float) -> float {
                return acc + curr;
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
    pub static ref STRING_SUM: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        r#"(iter: () -> (bool, string)) -> string {
            return iter $"" (acc: string, curr: string) -> string {
                return acc + curr;
            }
        }"#
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Sum;
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

pub fn exec(var: Variable) -> ExecResult {
    let return_type = var.as_type();
    if return_type.matches(&var_type!(() -> (bool, int))) {
        Ok(INT_SUM.exec_with_args(&[var])?)
    } else if return_type.matches(&var_type!(() -> (bool, float))) {
        Ok(FLOAT_SUM.exec_with_args(&[var])?)
    } else {
        Ok(STRING_SUM.exec_with_args(&[var])?)
    }
}
