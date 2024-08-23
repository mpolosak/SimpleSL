use crate as simplesl;
use crate::instruction::bitwise_or;
use crate::instruction::unary_operation::{UnaryOperation, UnaryOperator};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::{
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::{var, var_type};

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    match array.instruction {
        Instruction::Variable(Variable::Array(array))
            if array.element_type().matches(&var_type!(int)) =>
        {
            Ok(calc(&array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat.value.return_type().matches(&(var_type!(int))) =>
        {
            Ok(array_repeat.value.instruction.clone())
        }
        Instruction::Array(array) if array.element_type.matches(&var_type!(int)) => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(bitwise_or::create_from_instructions)
            .unwrap()),
        instruction if instruction.return_type().matches(&var_type!([int])) => Ok(UnaryOperation {
            instruction,
            op: UnaryOperator::BitOr,
        }
        .into()),
        ins => Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$||",
            expected: var_type!([int]),
            given: ins.return_type(),
        }),
    }
}

fn calc(array: &Array) -> Variable {
    let sum = array
        .iter()
        .map(|var| var.as_int().unwrap())
        .fold(0, |acc, curr| acc | curr);
    var!(sum)
}

pub fn recreate(instruction: Instruction) -> Instruction {
    if let Instruction::Variable(Variable::Array(array)) = &instruction {
        return calc(array).into();
    }
    UnaryOperation {
        instruction,
        op: UnaryOperator::BitOr,
    }
    .into()
}

pub fn exec(var: Variable) -> Variable {
    let array = var.into_array().unwrap();
    calc(&array)
}
