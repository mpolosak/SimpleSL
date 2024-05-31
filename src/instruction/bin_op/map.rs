use super::Map;
use crate as simplesl;
use crate::{
    function::Function,
    instruction::{
        traits::{CanBeUsed, ExecResult},
        Exec,
    },
    interpreter::Interpreter,
    variable::{self, Array, FunctionType, ReturnType, Type, Variable},
};
use simplesl_macros::var_type;
use std::{iter, sync::Arc};

impl CanBeUsed for Map {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        if let Some(types) = lhs.clone().flatten_tuple() {
            return Self::can_be_used_zip(&types, rhs);
        }
        let Some(element_type) = lhs.index_result() else {
            return false;
        };
        let element_type2 = element_type.clone();
        let expected_function = var_type!((element_type)->any | (int, element_type2)->any);
        rhs.matches(&expected_function)
    }
}

impl Map {
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
        let expected_function = FunctionType {
            params,
            return_type: Type::Any,
        } | FunctionType {
            params: extended_params.into(),
            return_type: Type::Any,
        };
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
                    let args: Box<[Variable]> =
                        arrays.iter().map(|array| array[i].clone()).collect();
                    function.exec(&args)
                })
                .collect::<Result<_, _>>()
        } else {
            (0..len)
                .map(|i| {
                    let args: Box<[Variable]> = iter::once(Variable::from(i))
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
}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?.into_function().unwrap();
        if let Variable::Tuple(arrays) = array {
            return Self::zip_map(arrays, function);
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
}

impl ReturnType for Map {
    fn return_type(&self) -> Type {
        let element_type = self.rhs.return_type().return_type().unwrap();
        var_type!([element_type])
    }
}

#[cfg(test)]
mod tests {
    use crate as simplesl;
    use crate::instruction::{bin_op::Map, traits::CanBeUsed};
    use simplesl_macros::var_type;

    #[test]
    fn can_be_used() {
        // true
        assert!(Map::can_be_used(&var_type!([int]), &var_type!((int)->int)));
        assert!(Map::can_be_used(
            &var_type!([int]),
            &var_type!((int|float)->any)
        ));
        assert!(Map::can_be_used(
            &var_type!([int]),
            &var_type!((int, any)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!([int]),
            &var_type!((any, any)->float)
        ));
        assert!(Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int | float)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int | float)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, any)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!([int]),
            &var_type!((int)->int | (any)->any)
        ));
        assert!(Map::can_be_used(
            &var_type!([int]),
            &var_type!((int, any)->int | (any)->any)
        ));
        assert!(Map::can_be_used(
            &var_type!([int]),
            &var_type!((any, any)->float | (int | float)->int)
        ));
        assert!(Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int | float)->int | (int | float)->float)
        ));
        assert!(Map::can_be_used(
            &var_type!(([int], [float], [string])),
            &var_type!((int, float, string) -> any)
        ));
        assert!(Map::can_be_used(
            &var_type!(([int], [float] | [string | int], [string])),
            &var_type!(
                (int, int|float|string, string) -> float | (int | float, any, any, any) -> any
            )
        ));
        assert!(Map::can_be_used(
            &var_type!(([int], [float]) | ([string], [any])),
            &var_type!((int | string, any) -> any | (any, any, any) -> ())
        ));
        // false
        assert!(!Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((int, int | float, any)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!([int] | [float]),
            &var_type!((any, float)->int)
        ));
        assert!(!Map::can_be_used(
            &var_type!([int]),
            &var_type!((int)->int | int)
        ));
        assert!(!Map::can_be_used(
            &var_type!([int] | float),
            &var_type!((int, any)->int | (any)->any)
        ));
        assert!(!Map::can_be_used(
            &var_type!([int] | ([int], [float])),
            &var_type!((any, any)->float | (int | float)->int)
        ));
    }
}
