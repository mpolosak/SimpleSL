use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::ExecResult;
use crate::variable::{Type, Variable};
use crate::{self as simplesl, Interpreter};
use crate::{
    instruction::{Instruction, InstructionWithStr},
    unary_operator::UnaryOperator,
    variable::ReturnType,
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, int));
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

pub fn and(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    let mut result = !0;
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        if let Variable::Int(value) = tuple[1] {
            result &= value;
        }
    }
    Ok(Variable::Int(result))
}

pub fn or(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    let mut result = 0;
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        if let Variable::Int(value) = tuple[1] {
            result |= value;
        }
    }
    Ok(Variable::Int(result))
}
