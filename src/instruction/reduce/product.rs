use crate::{
    self as simplesl,
    instruction::{
        multiply, unary_operation::UnaryOperation, ExecResult, Instruction, InstructionWithStr,
    },
    unary_operator::UnaryOperator,
    variable::{ReturnType, Type, Variable},
    Error, Interpreter,
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

pub fn exec(var: Variable, interpreter: &mut Interpreter) -> ExecResult {
    let iter = var.into_function().unwrap();
    let mut result = if iter.return_type().matches(&var_type!((bool, int))) {
        Variable::Int(1)
    } else {
        Variable::Float(1.0)
    };
    while let Variable::Tuple(tuple) = iter.exec(interpreter)? {
        if tuple[0] == Variable::Bool(false) {
            break;
        };
        result = multiply::exec(result, tuple[1].clone());
    }
    Ok(result)
}
