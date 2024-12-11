use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::ExecResult;
use crate::unary_operator::UnaryOperator;
use crate::variable::{ReturnType, Variable};
use crate::{self as simplesl, Interpreter};
use crate::{
    instruction::{Instruction, InstructionWithStr},
    variable::Type,
    Error,
};
use lazy_static::lazy_static;
use simplesl_macros::var_type;

lazy_static! {
    pub static ref ACCEPTED_TYPE: Type = var_type!(() -> (bool, any));
}

pub(crate) fn create(lhs: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Collect;
    let return_type = lhs.return_type();
    if !can_be_used(&return_type) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: lhs.str,
            op,
            expected: ACCEPTED_TYPE.clone(),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: lhs.instruction,
        op,
    }
    .into())
}

pub fn can_be_used(lhs: &Type) -> bool {
    lhs.matches(&ACCEPTED_TYPE)
}

pub(crate) fn exec(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    let mut vec = Vec::new();
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        vec.push(tuple[1].clone());
    }
    return Ok(vec.into());
}

pub(crate) fn return_type(lhs: Type) -> Type {
    lhs.return_type().unwrap().flatten_tuple().unwrap()[1].clone()
}
