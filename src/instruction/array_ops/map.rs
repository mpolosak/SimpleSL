use crate::{
    function::Function,
    instruction::{
        macros::binOp,
        traits::{CanBeUsed, ExecResult, ExecStop},
        Exec, Instruction,
    },
    interpreter::Interpreter,
    variable::{Array, FunctionType, ReturnType, Type, Variable},
};
use std::rc::Rc;

binOp!(Map, "@", cfi);

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
            .collect::<Option<Box<[Type]>>>()
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

    fn zip_map(arrays: Rc<[Variable]>, function: Rc<Function>) -> ExecResult {
        let arrays: Box<[&Rc<Array>]> = arrays
            .iter()
            .map(|array| {
                let Variable::Array(array) = array else {
                    unreachable!()
                };
                array
            })
            .collect();
        let len = arrays.iter().map(|array| array.len()).min().unwrap();
        if function.params.len() == arrays.len() {
            let array = (0..len)
                .map(|i| {
                    let args: Box<[Variable]> =
                        arrays.iter().map(|array| array[i].clone()).collect();
                    function.exec(&args)
                })
                .collect::<Result<Variable, ExecError>>()?;
            return Ok(array);
        }
        let array = (0..len)
            .map(|i| {
                let mut args = vec![i.into()];
                args.extend(arrays.iter().map(|array| array[i].clone()));
                function.exec(&args)
            })
            .collect::<Result<Variable, ExecError>>()?;
        Ok(array)
    }
}

impl Exec for Map {
    fn exec(&self, interpreter: &mut Interpreter) -> ExecResult {
        let array = self.lhs.exec(interpreter)?;
        let function = self.rhs.exec(interpreter)?;
        let Variable::Function(function) = function else {
            panic!("Tried to do {array} @ {function}")
        };
        if let Variable::Tuple(arrays) = array {
            return Self::zip_map(arrays, function);
        }
        let Variable::Array(array) = array else {
            panic!("Tried to do {array} @ {function}")
        };
        let iter = array.iter().cloned();
        if function.params.len() == 1 {
            return iter
                .map(|var| function.exec(&[var]))
                .collect::<Result<Variable, ExecError>>()
                .map_err(ExecStop::from);
        }
        iter.enumerate()
            .map(|(index, var)| function.exec(&[index.into(), var]))
            .collect::<Result<Variable, ExecError>>()
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
        instruction::{array_ops::Map, traits::CanBeUsed},
        variable::Type,
    };
    #[test]
    fn can_be_used() {
        let int_array = parse_type!("[int]");
        let float_array = parse_type!("[float]");
        // true
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("function(int)->int")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("function(int|float)->any")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("function(int, any)->int")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("function(any, any)->float")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("function(int | float)->int")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("function(int, int | float)->int")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("function(any, any)->int")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("function(int)->int | function(any)->any")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("function(int, any)->int | function(any)->any")
        ));
        assert!(Map::can_be_used(
            &int_array,
            &parse_type!("function(any, any)->float | function(int | float)->int")
        ));
        assert!(Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("function(int, int | float)->int | function(int | float)->float")
        ));
        assert!(Map::can_be_used(
            &parse_type!("([int], [float], [string])"),
            &parse_type!("function(int, float, string) -> any")
        ));
        assert!(Map::can_be_used(
            &parse_type!("([int], [float]|[string|int], [string])"),
            &parse_type!(
                "function(int, int|float|string, string) -> float\
            | function(int | float, any, any, any) -> any"
            )
        ));
        assert!(Map::can_be_used(
            &parse_type!("([int], [float])|([string], [any])"),
            &parse_type!("function(int | string, any) -> any | function(any, any, any) -> ()")
        ));
        // false
        assert!(!Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("function(int, int)->int")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("function(int, int | float, any)->int")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | float_array.clone()),
            &parse_type!("function(any, float)->int")
        ));
        assert!(!Map::can_be_used(
            &int_array,
            &parse_type!("function(int)->int | int")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | Type::Float),
            &parse_type!("function(int, any)->int | function(any)->any")
        ));
        assert!(!Map::can_be_used(
            &(int_array.clone() | Type::from((int_array.clone(), float_array.clone()))),
            &parse_type!("function(any, any)->float | function(int | float)->int")
        ));
    }
}
