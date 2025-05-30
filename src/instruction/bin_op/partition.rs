use crate::{
    self as simplesl,
    instruction::ExecResult,
    variable::{Array, Type, Typed, Variable},
};
use simplesl_macros::{var, var_type};

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
    let element_type2 = element_type.clone();
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

pub fn return_type(var_type: Type) -> Type {
    let element_type = var_type.iter_element().unwrap();
    let element_type2 = element_type.clone();
    var_type!(([element_type], [element_type2]))
}
