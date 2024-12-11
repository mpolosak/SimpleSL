use crate as simplesl;
use crate::variable::Typed;
use crate::{
    instruction::ExecResult,
    variable::{Array, Variable},
};
use simplesl_macros::var;

pub fn exec(iter: Variable, function: Variable) -> ExecResult {
    let (Variable::Function(iter), Variable::Function(function)) = (&iter, &function) else {
        unreachable!("Tried to do {iter} ? {function}")
    };
    let mut left = Vec::new();
    let mut right = Vec::new();
    while let Variable::Tuple(tuple) = iter.exec_with_args(&[])? {
        if tuple[0] == Variable::Bool(false) {
            break;
        }
        let element = tuple[1].clone();
        match function.exec_with_args(&[element.clone()])? {
            Variable::Bool(true) => left.push(element),
            _ => right.push(element),
        };
    }

    let element_type = iter.as_type().iter_element().unwrap();
    let element_type2 = iter.as_type().iter_element().unwrap();
    let left = Array {
        element_type,
        elements: left.into(),
    };

    let right = Array {
        element_type: element_type2,
        elements: right.into(),
    };

    Ok(var!((left, right)))
}
