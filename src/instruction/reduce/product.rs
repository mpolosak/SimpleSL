use crate::{
    self as simplesl,
    instruction::{
        array_repeat::ArrayRepeat,
        multiply,
        postfix_op::{PostfixOperation, PostfixOperator},
        pow, ExecResult, Instruction, InstructionWithStr,
    },
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
            Ok(calc_int(&array).into())
        }
        Instruction::Variable(Variable::Array(array))
            if array.element_type() == &var_type!(float) =>
        {
            Ok(calc_float(&array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat
                .value
                .return_type()
                .matches(&var_type!(int | float)) =>
        {
            let ArrayRepeat { value, len } = Arc::unwrap_or_clone(array_repeat.clone());
            pow::create_from_instructions(value.instruction, len.instruction).map_err(Error::from)
        }
        Instruction::Array(array) if array.element_type.matches(&var_type!(int | float)) => {
            Ok(array
                .instructions
                .iter()
                .cloned()
                .map(|iws| iws.instruction)
                .reduce(|acc, curr| multiply::create_from_instructions(acc, curr))
                .unwrap())
        }
        instruction
            if instruction
                .return_type()
                .matches(&(var_type!([int] | [float]))) =>
        {
            Ok(PostfixOperation {
                instruction: array,
                op: PostfixOperator::Product,
            }
            .into())
        }
        ins => Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$*",
            expected: var_type!([float] | [int]),
            given: ins.return_type(),
        }),
    }
}

fn calc(array: &Array) -> Variable {
    match array.element_type() {
        var_type!(int) => calc_int(&array),
        var_type!(float) => calc_float(&array),
        element_type => unreachable!("Tried to calculate product of [{element_type}]"),
    }
}

fn calc_int(array: &Array) -> Variable {
    let product: i64 = array.iter().map(|var| var.as_int().unwrap()).product();
    var!(product)
}

fn calc_float(array: &Array) -> Variable {
    let product: f64 = array.iter().map(|var| var.as_float().unwrap()).product();
    var!(product)
}

pub fn recreate(instruction: InstructionWithStr) -> Result<Instruction, ExecError> {
    if let Instruction::Variable(Variable::Array(array)) = &instruction.instruction {
        return Ok(calc(array).into());
    }
    Ok(PostfixOperation {
        instruction,
        op: PostfixOperator::Product,
    }
    .into())
}

pub fn exec(var: Variable) -> ExecResult {
    let Variable::Array(array) = var else {
        unreachable!("Tried to sum not array")
    };
    Ok(calc(&array))
}
