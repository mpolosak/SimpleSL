use super::Map;
use crate::{
    function::Function,
    instruction::{
        traits::{CanBeUsed, ExecResult, ExecStop},
        Exec,
    },
    interpreter::Interpreter,
    variable::{Array, FunctionType, ReturnType, Type, Variable},
};
use std::{iter, sync::Arc};

impl CanBeUsed for Map {
    fn can_be_used(lhs: &Type, rhs: &Type) -> bool {
        if let Some(types) = lhs.clone().flatten_tuple() {
            return Self::can_be_used_zip(&types, rhs);
        }
        let Some(element_type) = lhs.index_result() else {
            return false;
        };
        let expected_function = FunctionType {
            params: [element_type.clone()].into(),
            return_type: Type::Any,
        } | FunctionType {
            params: [Type::Int, element_type].into(),
            return_type: Type::Any,
        };
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
        if function.params.len() == arrays.len() {
            return (0..len)
                .map(|i| {
                    let args: Box<[Variable]> =
                        arrays.iter().map(|array| array[i].clone()).collect();
                    function.exec(&args)
                })
                .collect::<Result<_, _>>()
                .map_err(ExecStop::from);
        }
        (0..len)
            .map(|i| {
                let args: Box<[Variable]> = iter::once(Variable::from(i))
                    .chain(arrays.iter().map(|array| array[i].clone()))
                    .collect();
                function.exec(&args)
            })
            .collect::<Result<_, _>>()
            .map_err(ExecStop::from)
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
        if function.params.len() == 1 {
            return iter
                .map(|var| function.exec(&[var]))
                .collect::<Result<_, _>>()
                .map_err(ExecStop::from);
        }
        iter.enumerate()
            .map(|(index, var)| function.exec(&[index.into(), var]))
            .collect::<Result<_, _>>()
            .map_err(ExecStop::from)
    }
}

impl ReturnType for Map {
    fn return_type(&self) -> Type {
        [self.rhs.return_type().return_type().unwrap()].into()
    }
}

#[cfg(test)]
mod tests {
    use crate::variable::parse_type;
    use crate::{
        instruction::{bin_op::Map, traits::CanBeUsed},
        variable::Type,
    };
    #[test]
    fn can_be_used() {
        let int_array = parse_type!("[int]");
        let float_array = parse_type!("[float]");
        // true
        assert!(Map::can_be_used(&int_array, &parse_type!("(int)->int")));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("(int|float)->any")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("(int, any)->int")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("(any, any)->float")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("(int | float)->int")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("(int, int | float)->int")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("(any, any)->int")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("(int)->int | (any)->any")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("(int, any)->int | (any)->any")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("(any, any)->float | (int | float)->int")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("(int, int | float)->int | (int | float)->float")
        ));
        assert!(Map::can_be_used(
            &parse_type!("([int], [float], [string])"),
            &parse_type!("(int, float, string) -> any")
        ));
        assert!(Map::can_be_used(
            &parse_type!("([int], [float]|[string|int], [string])"),
            &parse_type!(
                "(int, int|float|string, string) -> float | (int | float, any, any, any) -> any"
            )
        ));
        assert!(Map::can_be_used(
            &parse_type!("([int], [float])|([string], [any])"),
            &parse_type!("(int | string, any) -> any | (any, any, any) -> ()")
        ));
        // false
        assert!(!Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("(int, int)->int")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("(int, int | float, any)->int")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("(any, float)->int")
        ));
        assert!(!Map::can_be_used(
            &int_array,
            &parse_type!("(int)->int | int")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | Type::Float),
            &parse_type!("(int, any)->int | (any)->any")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | Type::from((int_array.clone(), float_array.clone()))),
            &parse_type!("(any, any)->float | (int | float)->int")
        ));
    }
}
