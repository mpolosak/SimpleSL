use crate as simplesl;
use crate::instruction::bitwise_and;
use crate::instruction::unary_operation::{UnaryOperation, UnaryOperator};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::variable::Type;
use crate::{
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::var_type;

use super::all;

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([int] | [bool])) {
        return Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$&",
            expected: var_type!([int] | [bool]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op: UnaryOperator::BitAnd,
    }
    .into())
}

fn calc(array: &Array) -> Variable {
    if array.element_type == Type::Bool {
        return all::calc(array);
    }
    array
        .iter()
        .map(|var| var.as_int().unwrap())
        .fold(!0, |acc, curr| acc & curr)
        .into()
}

pub fn recreate(instruction: Instruction) -> Instruction {
    match instruction {
        Instruction::Variable(Variable::Array(array)) => calc(&array).into(),
        Instruction::ArrayRepeat(array_repeat) => array_repeat.value.instruction.clone(),
        Instruction::Array(array) => array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(bitwise_and::create_from_instructions)
            .unwrap(),
        instruction => UnaryOperation {
            instruction,
            op: UnaryOperator::BitAnd,
        }
        .into(),
    }
}
pub fn exec(var: Variable) -> Variable {
    let array = var.into_array().unwrap();
    calc(&array)
}
