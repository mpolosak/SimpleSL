use crate as simplesl;
use crate::instruction::or;
use crate::instruction::unary_operation::UnaryOperation;
use crate::instruction::{Instruction, InstructionWithStr};
use crate::unary_operator::UnaryOperator;
use crate::{
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::var_type;

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([bool])) {
        return Err(Error::IncorectUnaryOperatorOperand {
            ins: array.str,
            op: UnaryOperator::Any,
            expected: var_type!([bool]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op: UnaryOperator::Any,
    }
    .into())
}

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
