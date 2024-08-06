use crate as simplesl;
use crate::instruction::postfix_op::{PostfixOperation, PostfixOperator};
use crate::instruction::{
    add, array_repeat::ArrayRepeat, multiply, ExecResult, Instruction, InstructionWithStr,
};
use crate::{
    variable::{Array, ReturnType, Variable},
    Error, ExecError,
};
use simplesl_macros::{var, var_type};
use std::sync::Arc;

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    match &array.instruction {
        Instruction::Variable(Variable::Array(array))
            if array.element_type().matches(&var_type!(int)) =>
        {
            Ok(calc_int(array).into())
        }
        Instruction::Variable(Variable::Array(array))
            if array.element_type() == &var_type!(float) =>
        {
            Ok(calc_float(array).into())
        }
        Instruction::Variable(Variable::Array(array))
            if array.element_type() == &var_type!(string) =>
        {
            Ok(calc_string(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat
                .value
                .return_type()
                .matches(&var_type!(int | float)) =>
        {
            let ArrayRepeat { value, len } = Arc::unwrap_or_clone(array_repeat.clone());
            Ok(multiply::create_from_instructions(
                value.instruction,
                len.instruction,
            ))
        }
        Instruction::Array(array)
            if array.element_type.matches(&var_type!(int | float | string)) =>
        {
            Ok(array
                .instructions
                .iter()
                .cloned()
                .map(|iws| iws.instruction)
                .reduce(|acc, curr| add::create_from_instructions(acc, curr))
                .unwrap())
        }
        instruction
            if instruction
                .return_type()
                .matches(&var_type!([int] | [float] | [string])) =>
        {
            Ok(PostfixOperation {
                instruction: array,
                op: PostfixOperator::Sum,
            }
            .into())
        }
        ins => Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$+",
            expected: var_type!([int] | [float] | [string]),
            given: ins.return_type(),
        }),
    }
}

fn calc(array: &Array) -> Variable {
    match array.element_type() {
        var_type!(int) => calc_int(&array),
        var_type!(float) => calc_float(&array),
        var_type!(string) => calc_string(&array),
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

pub fn recreate(instruction: InstructionWithStr) -> Result<Instruction, ExecError> {
    if let Instruction::Variable(Variable::Array(array)) = &instruction.instruction {
        return Ok(calc(array).into());
    }
    Ok(PostfixOperation {
        instruction,
        op: PostfixOperator::Sum,
    }
    .into())
}

pub fn exec(var: Variable) -> ExecResult {
    let Variable::Array(array) = var else {
        unreachable!("Tried to sum not array")
    };
    Ok(calc(&array))
}
