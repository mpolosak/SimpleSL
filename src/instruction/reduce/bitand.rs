use super::all;
use crate::instruction::bitwise_and;
use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::Instruction;
use crate::unary_operator::UnaryOperator;
use crate::variable::Type;
use crate::variable::{Array, Variable};

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
