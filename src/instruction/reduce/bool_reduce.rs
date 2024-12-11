use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::variable::ReturnType;
use crate::{self as simplesl, Error};
use crate::{
    instruction::ExecResult,
    variable::{Type, Variable},
    Interpreter,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, bool));
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

pub fn all(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        if tuple[1] == Variable::Bool(false) {
            return Ok(Variable::Bool(false));
        }
    }
    Ok(Variable::Bool(true))
}

pub fn any(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        if tuple[1] == Variable::Bool(true) {
            return Ok(Variable::Bool(true));
        }
    }
    Ok(Variable::Bool(false))
}
