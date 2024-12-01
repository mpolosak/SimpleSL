use crate as simplesl;
use crate::instruction::unary_operation::{UnaryOperation, UnaryOperator};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::variable::Type;
use crate::{
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::var_type;

use super::any;

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([int] | [bool])) {
        return Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$|",
            expected: var_type!([int] | [bool]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op: UnaryOperator::BitOr,
    }
    .into())
}

fn calc(array: &Array) -> Variable {
    if array.element_type() == &Type::Bool {
        return any::calc(array);
    }
    array
        .iter()
        .map(|var| var.as_int().unwrap())
        .fold(0, |acc, curr| acc | curr)
        .into()
}

pub fn recreate(instruction: Instruction) -> Instruction {
    match instruction {
        Instruction::Variable(Variable::Array(array)) => calc(&array).into(),
        instruction => UnaryOperation {
            instruction,
            op: UnaryOperator::BitOr,
        }
        .into(),
    }
}

pub fn exec(var: Variable) -> Variable {
    let array = var.into_array().unwrap();
    calc(&array)
}
