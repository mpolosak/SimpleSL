use crate::{
    self as simplesl,
    function::Function,
    instruction::{unary_operation::UnaryOperation, ExecResult, Instruction, InstructionWithStr},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed, Variable},
    Code, Error, Interpreter,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int) | () -> (bool, float));
}

lazy_static! {
    pub static ref INT_PRODUCT: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, int)) -> int {
            return iter $1 (acc: int, curr: int) -> int {
                return acc * curr;
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
    pub static ref FLOAT_PRODUCT: Arc<Function> = Code::parse(
        &Interpreter::without_stdlib(),
        "(iter: () -> (bool, float)) -> float {
            return iter $1.0 (acc: float, curr: float) -> float {
                return acc * curr;
            }
        }"
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Product;
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
        return Ok(INT_PRODUCT.exec_with_args(&[var])?);
    }
    Ok(FLOAT_PRODUCT.exec_with_args(&[var])?)
}
