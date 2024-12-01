use crate::{
    self as simplesl,
    instruction::{
        multiply,
        unary_operation::{UnaryOperation, UnaryOperator},
        Instruction, InstructionWithStr,
    },
    variable::{Array, ReturnType, Variable},
    Error,
};
use simplesl_macros::{var, var_type};

pub fn create(array: InstructionWithStr) -> Result<Instruction, Error> {
    let return_type = array.return_type();
    if !return_type.matches(&var_type!([float] | [int])) {
        return Err(Error::IncorectPostfixOperatorOperand {
            ins: array.str,
            op: "$*",
            expected: var_type!([float] | [int]),
            given: return_type,
        });
    }
    Ok(UnaryOperation {
        instruction: array.instruction,
        op: UnaryOperator::Product,
    }
    .into())
}

fn calc(array: &Array) -> Variable {
    match array.element_type() {
        var_type!(int) => calc_int(array),
        var_type!(float) => calc_float(array),
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

pub fn recreate(instruction: Instruction) -> Instruction {
    match instruction {
        Instruction::Variable(Variable::Array(array)) => calc(&array).into(),
        Instruction::Array(array) => array
            .instructions
            .iter()
            .cloned()
            .map(|iws| iws.instruction)
            .reduce(multiply::create_from_instructions)
            .unwrap(),
        instruction => UnaryOperation {
            instruction,
            op: UnaryOperator::Product,
        }
        .into(),
    }
}

pub fn exec(var: Variable) -> Variable {
    let array = var.into_array().unwrap();
    calc(&array)
}
