use simplesl_macros::export_function;

use crate::function::{Function, NativeFunction, Param, Params};
use crate::intepreter::Intepreter;
use crate::variable_type::Type;
use crate::{
    error::Error,
    intepreter::VariableMap,
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

    variables.add_native_function(
        "for_each",
        NativeFunction {
            params: Params {
                standard: params!("array": Type::Array, "function": Type::Function{ 
                    return_type: Type::Any.into(), params: vec![Type::Any], catch_rest: false}),
                catch_rest: None,
            },
            return_type: Type::Array,
            body: |_name, intepreter, args| {
                let Variable::Array(array) = args.get("array")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                let mut new_array = Array::new();
                for var in array.iter() {
                    new_array.push(function.exec("function", intepreter, vec![var.clone()])?);
                }
                Ok(Variable::Array(new_array.into()))
            },
        },
    );
    variables.add_native_function(
        "filter",
        NativeFunction {
            params: Params {
                standard: params!(
                    "array": Type::Array,
                    "function": Type::Function{
                        return_type:Type::Int.into(),
                        params: vec![Type::Any],
                        catch_rest: false
                    }
                ),
                catch_rest: None,
            },
            return_type: Type::Array,
            body: |_name, interpreter, args| {
                let Variable::Array(array) = args.get("array")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                let mut new_array = Array::new();
                for element in array.iter() {
                    if function.exec("function", interpreter, vec![element.clone()])?
                        != Variable::Int(0)
                    {
                        new_array.push(element.clone());
                    }
                }
                Ok(Variable::Array(new_array.into()))
            },
        },
    );
    variables.add_native_function(
        "reduce",
        NativeFunction {
            params: Params {
                standard: params!(
                    "array": Type::Array,
                    "initial_value": Type::Any,
                    "function": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![Type::Any, Type::Any],
                        catch_rest: false
                    }
                ),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Array(array) = args.get("array")? else {
                    panic!()
                };
                let initial_value = args.get("initial_value")?;
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                array.iter().try_fold(initial_value, |acc, current| {
                    function.exec("function", intepreter, vec![acc, current.clone()])
                })
            },
        },
    );

    #[export_function("zip")]
    fn array_zip(array1: Rc<Array>, array2: Rc<Array>) -> Rc<Array> {
        let new_array: Array = zip(array1.iter(), array2.iter())
            .map(|(element1, element2)| {
                Variable::Array(vec![element1.clone(), element2.clone()].into())
            })
            .collect();
        new_array.into()
    }

    fn recsub(
        intepreter: &mut Intepreter,
        n: usize,
        array: Rc<Array>,
        function: Rc<dyn Function>,
    ) -> Result<Variable, Error> {
        if n < array.len() {
            Ok(array[n].clone())
        } else {
            let result = function.exec("function", intepreter, (*array).clone())?;
            let mut new_array = (*array).clone();
            new_array.remove(0);
            new_array.push(result);
            recsub(intepreter, n - 1, new_array.into(), function)
        }
    }

    variables.add_native_function(
        "recsub",
        NativeFunction {
            params: Params {
                standard: params!(
                    "array": Type::Array,
                    "function": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![Type::Any],
                        catch_rest: false
                    },
                    "n": Type::Int
                ),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Array(array) = args.get("array")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                let Variable::Int(n) = args.get("n")? else {
                    panic!();
                };
                if n < 0 {
                    return Err(Error::CannotBeNegative(String::from("n")));
                }
                let n = n as usize;
                recsub(intepreter, n, array, function)
            },
        },
    );
    variables.add_native_function(
        "arecsub",
        NativeFunction {
            params: Params {
                standard: params!(
                    "array": Type::Array,
                    "function": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![Type::Array],
                        catch_rest: false
                    },
                    "n": Type::Int
                ),
                catch_rest: None,
            },
            return_type: Type::Array,
            body: |_name, intepreter, args| {
                let Variable::Array(array) = args.get("array")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                let Variable::Int(n) = args.get("n")? else {
                    panic!();
                };
                if n < 0 {
                    return Err(Error::CannotBeNegative(String::from("n")));
                }
                let n = n as usize;
                if array.len() > n {
                    let new_array: Array = (*array).clone().into_iter().take(n).collect();
                    Ok(Variable::Array(new_array.into()))
                } else {
                    let mut new_array = (*array).clone();
                    for _ in 0..n - array.len() {
                        new_array.push(function.exec(
                            "function",
                            intepreter,
                            vec![Variable::Array(Rc::new(new_array.clone()))],
                        )?);
                    }
                    Ok(Variable::Array(new_array.into()))
                }
            },
        },
    );
}
