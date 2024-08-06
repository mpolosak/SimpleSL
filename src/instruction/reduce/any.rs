use crate as simplesl;
use crate::instruction::postfix_op::{PostfixOperation, PostfixOperator};
use crate::instruction::{or, ExecResult};
use crate::instruction::{Instruction, InstructionWithStr};
use crate::ExecError;
use crate::{
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::{var, var_type};

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    match &array.instruction {
        Instruction::Variable(Variable::Array(array))
            if array.element_type().matches(&var_type!(int)) =>
        {
            Ok(calc(array).into())
        }
        Instruction::ArrayRepeat(array_repeat)
            if array_repeat.value.return_type().matches(&var_type!(int)) =>
        {
            Ok(array_repeat.value.instruction.clone())
        }
        Instruction::Array(array) if array.element_type.matches(&var_type!(int)) => Ok(array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(|acc, curr| or::create_from_instructions(acc, curr))
            .unwrap()),
        instruction if instruction.return_type().matches(&var_type!([int])) => {
            Ok(PostfixOperation {
                instruction: array,
                op: PostfixOperator::Any,
            }
            .into())
        }
        ins => Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$||",
            expected: var_type!([int]),
            given: ins.return_type(),
        }),
    }
}

fn calc(array: &Array) -> Variable {
    let sum = array.iter().any(|var| *var.as_int().unwrap() != 0);
    var!(sum)
}

pub fn recreate(instruction: InstructionWithStr) -> Result<Instruction, ExecError> {
    if let Instruction::Variable(Variable::Array(array)) = &instruction.instruction {
        return Ok(calc(array).into());
    }
    Ok(PostfixOperation {
        instruction,
        op: PostfixOperator::Any,
    }
    .into())
}

pub fn exec(var: Variable) -> ExecResult {
    let array = var.into_array().unwrap();
    Ok(calc(&array).into())
}
