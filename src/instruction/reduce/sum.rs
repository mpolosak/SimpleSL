use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::ExecResult;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::stdlib::operators::{FLOAT_SUM, INT_SUM, STRING_SUM};
use crate::unary_operator::UnaryOperator;
use crate::variable::{Type, Typed};
use crate::{self as simplesl};
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
        Ok(INT_SUM.as_function().unwrap().exec_with_args(&[var])?)
    } else if return_type.matches(&var_type!(() -> (bool, float))) {
        Ok(FLOAT_SUM.as_function().unwrap().exec_with_args(&[var])?)
    } else {
        Ok(STRING_SUM.as_function().unwrap().exec_with_args(&[var])?)
    }
}
