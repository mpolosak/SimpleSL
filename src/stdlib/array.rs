use simplesl_macros::export_function;

use crate::function::{Function, NativeFunction, Param, Params};
use crate::interpreter::Interpreter;
use crate::variable_type::Type;
use crate::{
    error::Error,
    interpreter::VariableMap,
    params,
    variable::{Array, Variable},
};
use std::iter::zip;
use std::rc::Rc;

pub fn add_array_functions(variables: &mut VariableMap) {
    #[export_function]
    fn new_array(length: i64, value: Variable) -> Result<Rc<Array>, Error> {
        if length < 0 {
            return Err(Error::CannotBeNegative(String::from("length")));
        }
        let mut array = Array::new();
        for _ in 0..length {
            array.push(value.clone());
        }
        Ok(array.into())
    }

    #[export_function]
    fn array_at(array: Rc<Array>, index: i64) -> Result<Variable, Error> {
        if index < 0 {
            return Err(Error::CannotBeNegative(String::from("index")));
        }
        let index = index as usize;
        if index < array.len() {
            Ok(array[index].clone())
        } else {
            Err(Error::IndexToBig)
        }
    }

    #[export_function]
    fn array_concat(array1: Rc<Array>, array2: Rc<Array>) -> Rc<Array> {
        let mut new_array = (*array1).clone();
        for element in array2.iter() {
            new_array.push(element.clone());
        }
        new_array.into()
    }

    #[export_function]
    fn array_len(array: Rc<Array>) -> i64 {
        array.len() as i64
    }

    #[export_function]
    fn for_each(
        interpreter: &mut Interpreter,
        array: Rc<Array>,
        #[function(
            return_type: Type::Any.into(),
            params: vec![Type::Any],
            catch_rest: false
        )]
        function: Rc<dyn Function>,
    ) -> Result<Rc<Array>, Error> {
        let mut new_array = Array::new();
        for var in array.iter() {
            new_array.push(function.exec("function", interpreter, vec![var.clone()])?);
        }
        Ok(new_array.into())
    }

    #[export_function]
    fn filter(
        interpreter: &mut Interpreter,
        array: Rc<Array>,
        #[function(
            return_type:Type::Int.into(),
            params: vec![Type::Any],
            catch_rest: false
        )]
        function: Rc<dyn Function>,
    ) -> Result<Rc<Array>, Error> {
        let mut new_array = Array::new();
        for element in array.iter() {
            if function.exec("function", interpreter, vec![element.clone()])? != Variable::Int(0) {
                new_array.push(element.clone());
            }
        }
        Ok(new_array.into())
    }

    #[export_function]
    fn reduce(
        interpreter: &mut Interpreter,
        array: Rc<Array>,
        initial_value: Variable,
        #[function(
            return_type: Type::Any.into(),
            params: vec![Type::Any, Type::Any],
            catch_rest: false
        )]
        function: Rc<dyn Function>,
    ) -> Result<Variable, Error> {
        array.iter().try_fold(initial_value, |acc, current| {
            function.exec("function", interpreter, vec![acc, current.clone()])
        })
    }

    #[export_function(name = "zip")]
    fn array_zip(array1: Rc<Array>, array2: Rc<Array>) -> Rc<Array> {
        let new_array: Array = zip(array1.iter(), array2.iter())
            .map(|(element1, element2)| {
                Variable::Array(vec![element1.clone(), element2.clone()].into())
            })
            .collect();
        new_array.into()
    }

    #[export_function]
    fn recsub(
        interpreter: &mut Interpreter,
        array: Rc<Array>,
        #[function(
            return_type: Type::Any.into(),
            params: vec![Type::Array],
            catch_rest: false
        )]
        function: Rc<dyn Function>,
        n: i64,
    ) -> Result<Rc<Array>, Error> {
        if n < 0 {
            return Err(Error::CannotBeNegative(String::from("n")));
        }
        let n = n as usize;
        if array.len() > n {
            let new_array: Array = (*array).clone().into_iter().take(n).collect();
            Ok(new_array.into())
        } else {
            let mut new_array = (*array).clone();
            for _ in 0..n - array.len() {
                new_array.push(function.exec(
                    "function",
                    interpreter,
                    vec![Variable::Array(Rc::new(new_array.clone()))],
                )?);
            }
            Ok(new_array.into())
        }
    }
}
