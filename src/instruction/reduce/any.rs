use crate as simplesl;
use crate::instruction::or;
use crate::instruction::unary_operation::{UnaryOperation, UnaryOperator};
use crate::instruction::{Instruction, InstructionWithStr};
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
            op: "$||",
            expected: var_type!([bool]),
            given: return_type,
        });
    }
    match array.instruction {
        Instruction::Variable(Variable::Array(array)) => Ok(calc(&array).into()),
        Instruction::ArrayRepeat(array_repeat) => Ok(array_repeat.value.instruction.clone()),
        Instruction::Array(array) => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(or::create_from_instructions)
            .unwrap()),
        instruction => Ok(UnaryOperation {
            instruction,
            op: UnaryOperator::Any,
        }
        .into()),
    }
}

fn calc(array: &Array) -> Variable {
    array.iter().any(|var| *var.as_bool().unwrap()).into()
}

pub fn recreate(instruction: Instruction) -> Instruction {
    if let Instruction::Variable(Variable::Array(array)) = &instruction {
        return calc(array).into();
    }
    UnaryOperation {
        instruction,
        op: UnaryOperator::Any,
    }
    .into()
}

pub fn exec(var: Variable) -> Variable {
    let array = var.into_array().unwrap();
    calc(&array)
}
