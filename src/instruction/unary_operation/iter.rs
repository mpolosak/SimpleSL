use super::UnaryOperation;
use crate::function::Function;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::variable::{ReturnType, Type, Typed, Variable};
use crate::{self as simplesl, Code, Error, Interpreter};
use lazy_static::lazy_static;
use simplesl_macros::var_type;
use std::sync::Arc;

lazy_static! {
    static ref ITER: Arc<Function> = Code::parse(
        &Interpreter::with_stdlib(),
        "(array: [int], default: int) -> () -> (bool, int) {
            i := mut -1;
            len := len(array)
            return () -> (bool, int) {
                i+=1;
                if *i < len {
                    return (true, array[*i])
                }
                return (false, default)
            }
        } "
    )
    .unwrap()
    .exec()
    .unwrap()
    .into_function()
    .unwrap();
}

pub(crate) fn exec(var: Variable) -> Variable {
    let element_type = var.as_type().element_type().unwrap();
    let default = Variable::of_type(&element_type).unwrap_or(Variable::Void);
    let result = ITER
        .exec_with_args(&[var, default])
        .unwrap()
        .into_function()
        .unwrap();
    let mut result = Arc::unwrap_or_clone(result);
    result.return_type = var_type!((bool, element_type));
    result.into()
}

pub(crate) fn return_type(lhs: Type) -> Type {
    let element_type = lhs.element_type().unwrap();
    var_type!(() -> (bool, element_type))
}

pub(crate) fn create(lhs: InstructionWithStr) -> Result<Instruction, Error> {
    let op = UnaryOperator::Iter;
    let lhs_type = lhs.return_type();
    if !lhs_type.matches(&var_type!([any])) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: lhs.str,
            op,
            expected: var_type!([any]),
            given: lhs_type,
        });
    }
    Ok(UnaryOperation {
        instruction: lhs.instruction,
        op,
    }
    .into())
}
