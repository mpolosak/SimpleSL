use crate::{
    self as simplesl,
    instruction::{unary_operation::UnaryOperation, ExecResult, Instruction, InstructionWithStr},
    stdlib::operators::{FLOAT_PRODUCT, INT_PRODUCT},
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Typed, Variable},
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int) | () -> (bool, float));
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
        return Ok(INT_PRODUCT.as_function().unwrap().exec_with_args(&[var])?);
    }
    Ok(FLOAT_PRODUCT
        .as_function()
        .unwrap()
        .exec_with_args(&[var])?)
}
