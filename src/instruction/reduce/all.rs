use crate as simplesl;
use crate::instruction::unary_operation::{UnaryOperation, UnaryOperator};
use crate::instruction::{and, Instruction, InstructionWithStr};
use crate::{
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::var_type;

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([bool])) {
        return Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$&&",
            expected: var_type!([bool]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op: UnaryOperator::All,
    }
    .into())
}

pub fn calc(array: &Array) -> Variable {
    array.iter().all(|var| *var.as_bool().unwrap()).into()
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
            .reduce(and::create_from_instructions)
            .unwrap(),
        instruction => UnaryOperation {
            instruction,
            op: UnaryOperator::All,
        }
        .into(),
    }
}

pub fn exec(var: Variable) -> Variable {
    let array = var.into_array().unwrap();
    calc(&array)
}
