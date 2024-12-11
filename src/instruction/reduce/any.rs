use crate::instruction::or;
use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::Instruction;
use crate::unary_operator::UnaryOperator;
use crate::variable::{Array, Variable};

pub fn calc(array: &Array) -> Variable {
    array.iter().any(|var| *var.as_bool().unwrap()).into()
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
            .reduce(or::create_from_instructions)
            .unwrap(),
        instruction => UnaryOperation {
            instruction,
            op: UnaryOperator::Any,
        }
        .into(),
    }
}

pub fn exec(var: Variable) -> Variable {
    let array = var.into_array().unwrap();
    calc(&array)
}
