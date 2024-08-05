use crate::instruction::{Instruction, InstructionWithStr};
use crate::{self as simplesl, Error};
use crate::{
    function::Function,
    instruction::traits::ExecResult,
    variable::{self, Array, ReturnType, Type, Variable},
};
use simplesl_macros::{var, var_type};
use std::{iter, sync::Arc};

use super::{BinOperation, BinOperator};

pub fn create_op(lhs: InstructionWithStr, rhs: InstructionWithStr) -> Result<Instruction, Error> {
    let lhs_type = lhs.return_type();
    let rhs_type = rhs.return_type();
    if !can_be_used(&lhs_type, &rhs_type) {
        return Err(Error::CannotDo2(lhs_type, stringify!(op), rhs_type));
    }
    Ok(BinOperation {
        lhs: lhs.instruction,
        rhs: rhs.instruction,
        op: BinOperator::Map,
    }
    .into())
}

fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
    if let Some(types) = lhs.clone().flatten_tuple() {
        return can_be_used_zip(&types, rhs);
    }
    let Some(element_type) = lhs.index_result() else {
        return false;
    };
    let element_type2 = element_type.clone();
    let expected_function = var_type!((element_type)->any | (int, element_type2)->any);
    rhs.matches(&expected_function)
}

fn can_be_used_zip(types: &[Type], rhs: &Type) -> bool {
    let Some(params) = types
        .iter()
        .map(Type::index_result)
        .collect::<Option<Arc<[Type]>>>()
    else {
        return false;
    };
    let mut extended_params = vec![Type::Int];
    extended_params.extend(params.iter().cloned());
    let extended_params = extended_params.into();
    let expected_function = var_type!(params -> any | extended_params -> any);
    rhs.matches(&expected_function)
}

fn zip_map(arrays: Arc<[Variable]>, function: Arc<Function>) -> ExecResult {
    let arrays: Box<[&Arc<Array>]> = arrays
        .iter()
        .map(|array| array.as_array().unwrap())
        .collect();
    let len = arrays.iter().map(|array| array.len()).min().unwrap();
    let elements = if function.params.len() == arrays.len() {
        (0..len)
            .map(|i| {
                let args: Box<[Variable]> = arrays.iter().map(|array| array[i].clone()).collect();
                function.exec(&args)
            })
            .collect::<Result<_, _>>()
    } else {
        (0..len)
            .map(|i| {
                let args: Box<[Variable]> = iter::once(var!(i))
                    .chain(arrays.iter().map(|array| array[i].clone()))
                    .collect();
                function.exec(&args)
            })
            .collect::<Result<_, _>>()
    }?;
    let element_type = function.return_type().into();
    Ok(variable::Array {
        element_type,
        elements,
    }
    .into())
}

pub fn exec(array: Variable, function: Variable) -> ExecResult {
    let function = function.into_function().unwrap();
    if let Variable::Tuple(arrays) = array {
        return zip_map(arrays, function);
    }
    let array = array.into_array().unwrap();
    let iter = array.iter().cloned();
    let elements = if function.params.len() == 1 {
        iter.map(|var| function.exec(&[var]))
            .collect::<Result<_, _>>()
    } else {
        iter.enumerate()
            .map(|(index, var)| function.exec(&[index.into(), var]))
            .collect::<Result<_, _>>()
    }?;
    let element_type = function.return_type();
    Ok(Array {
        element_type,
        elements,
    }
    .into())
}

pub fn return_type(rhs: Type) -> Type {
    let element_type = rhs.return_type().unwrap();
    var_type!([element_type])
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::instruction::bin_op::map;
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        // true
        assert!(map::can_be_used(&var_type!([int]), &var_type!((int)->int)));
        assert!(map::can_be_used(
            &var_type!([int]),
            &var_type!((int|float)->any)
        ));
        assert!(map::can_be_used(
            &var_type!([int]),
            &var_type!((int, any)->int)
        ));
        assert!(map::can_be_used(
            &var_type!([int]),
            &var_type!((any, any)->float)
        ));
        assert!(map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int | float)->int)
        ));
        assert!(map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int | float)->int)
        ));
        assert!(map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, any)->int)
        ));
        assert!(map::can_be_used(
            &var_type!([int]),
            &var_type!((int)->int | (any)->any)
        ));
        assert!(map::can_be_used(
            &var_type!([int]),
            &var_type!((int, any)->int | (any)->any)
        ));
        assert!(map::can_be_used(
            &var_type!([int]),
            &var_type!((any, any)->float | (int | float)->int)
        ));
        assert!(map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int | float)->int | (int | float)->float)
        ));
        assert!(map::can_be_used(
            &var_type!(([int], [float], [string])),
            &var_type!((int, float, string) -> any)
        ));
        assert!(map::can_be_used(
            &var_type!(([int], [float] | [string | int], [string])),
            &var_type!(
                (int, int|float|string, string) -> float | (int | float, any, any, any) -> any
            )
        ));
        assert!(map::can_be_used(
            &var_type!(([int], [float]) | ([string], [any])),
            &var_type!((int | string, any) -> any | (any, any, any) -> ())
        ));
        // false
        assert!(!map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int)->int)
        ));
        assert!(!map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int | float, any)->int)
        ));
        assert!(!map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, float)->int)
        ));
        assert!(!map::can_be_used(
            &var_type!([int]),
            &var_type!((int)->int | int)
        ));
        assert!(!map::can_be_used(
            &var_type!([int] | float),
            &var_type!((int, any)->int | (any)->any)
        ));
        assert!(!map::can_be_used(
            &var_type!([int] | ([int], [float])),
            &var_type!((any, any)->float | (int | float)->int)
        ));
    }
}
