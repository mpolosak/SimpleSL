use crate as simplesl;
use crate::{
    instruction::ExecResult,
    variable::{Array, Variable},
};
use simplesl_macros::var;

pub fn exec(array: Variable, function: Variable) -> ExecResult {
    let (Variable::Array(array), Variable::Function(function)) = (&array, &function) else {
        unreachable!("Tried to do {array} ? {function}")
    };
    let array_iter = array.iter().cloned();
    let mut left = Vec::new();
    let mut right = Vec::new();
    if function.params.len() == 1 {
        for element in array_iter {
            match function.exec_with_args(&[element.clone()])? {
                Variable::Bool(true) => left.push(element),
                _ => right.push(element),
            };
        }
    } else {
        for (index, element) in array_iter.enumerate() {
            match function.exec_with_args(&[index.into(), element.clone()])? {
                Variable::Bool(true) => left.push(element),
                _ => right.push(element),
            };
        }
    };
    let element_type = array.element_type().clone();
    let left = Array {
        element_type,
        elements: left.into(),
    };

    let element_type = array.element_type().clone();
    let right = Array {
        element_type,
        elements: right.into(),
    };

    Ok(var!((left, right)))
}
