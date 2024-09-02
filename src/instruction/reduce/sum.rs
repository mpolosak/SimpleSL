use crate as simplesl;
use crate::instruction::unary_operation::{UnaryOperation, UnaryOperator};
use crate::instruction::{
    add, array_repeat::ArrayRepeat, multiply, Instruction, InstructionWithStr,
};
use crate::{
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::{var, var_type};
use std::sync::Arc;

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([float] | [int] | [string])) {
        return Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$+",
            expected: var_type!([float] | [int] | [string]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op: UnaryOperator::Sum,
    }
    .into())
}

fn calc(array: &Array) -> Variable {
    match array.element_type() {
        var_type!(int) => calc_int(array),
        var_type!(float) => calc_float(array),
        var_type!(string) => calc_string(array),
        element_type => unreachable!("Tried to sum [{element_type}]"),
    }
}

fn calc_int(array: &Array) -> Variable {
    let sum = array.iter().map(|var| var.as_int().unwrap()).sum();
    Variable::Int(sum)
}

fn calc_float(array: &Array) -> Variable {
    let sum = array.iter().map(|var| var.as_float().unwrap()).sum();
    Variable::Float(sum)
}

fn calc_string(array: &Array) -> Variable {
    let sum: String = array
        .iter()
        .map(|var| var.as_string().unwrap())
        .fold(String::new(), |acc, curr| format!("{acc}{curr}"));
    var!(sum)
}

pub fn recreate(instruction: Instruction) -> Instruction {
    match instruction {
        Instruction::Variable(Variable::Array(array)) => calc(&array).into(),
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat
                .value
                .return_type()
                .matches(&var_type!(int | float)) =>
        {
            let ArrayRepeat { value, len } = Arc::unwrap_or_clone(array_repeat.clone());
            multiply::create_from_instructions(value.instruction, len.instruction)
        }
        Instruction::Array(array) => array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(add::create_from_instructions)
            .unwrap(),
        instruction => UnaryOperation {
            instruction,
            op: UnaryOperator::Sum,
        }
        .into(),
    }
}

pub fn exec(var: Variable) -> Variable {
    let Variable::Array(array) = var else {
        unreachable!("Tried to sum not array")
    };
    calc(&array)
}
