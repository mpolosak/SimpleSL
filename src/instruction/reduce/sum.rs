use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::ExecResult;
use crate::instruction::{add, Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::variable::Type;
use crate::{self as simplesl, Interpreter};
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

pub fn exec(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    let return_type = iter.return_type();
    let mut result = if return_type.matches(&var_type!((bool, int))) {
        Variable::Int(1)
    } else if return_type.matches(&var_type!((bool, float))) {
        Variable::Float(1.0)
    } else {
        Variable::String("".into())
    };
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        result = add::exec(result, tuple[1].clone());
    }
    Ok(result)
}
