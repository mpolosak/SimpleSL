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
    variables.add_native_function(
        "new_array",
        NativeFunction {
            params: Params {
                standard: params!("length": Type::Float, "value": Type::Any),
                catch_rest: None,
            },
            return_type: Type::Array,
            body: |_name, _intepreter, args| {
                let Variable::Float(len) = args.get("length")? else {
                    panic!();
                };
                if !is_natural(len) {
                    return Err(Error::WrongType(
                        String::from("lenght"),
                        /*Fix it later String::from("natural")*/ Type::Float,
                    ));
                }
                let len = len as usize;
                let value = args.get("value")?;
                let mut array = Array::new();
                for _ in 0..len {
                    array.push(value.clone());
                }
                Ok(Variable::Array(array.into()))
            },
        },
    );
    variables.add_native_function(
        "array_at",
        NativeFunction {
            params: Params {
                standard: params!("array": Type::Array, "index": Type::Float),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, _intepreter, args| {
                let Variable::Array(array) = args.get("array")? else {
                    panic!();
                };
                let Variable::Float(index) = args.get("index")? else {
                    panic!();
                };
                if !is_natural(index) {
                    return Err(Error::WrongType(
                        String::from("index"),
                        /*Fix it later String::from("natural")*/ Type::Float,
                    ));
                }
                let index = index as usize;
                if index < array.len() {
                    Ok(array[index].clone())
                } else {
                    Err(Error::IndexToBig)
                }
            },
        },
    );
    variables.add_native_function(
        "array_concat",
        NativeFunction {
            params: Params {
                standard: params!("array1": Type::Array, "array2": Type::Array),
                catch_rest: None,
            },
            return_type: Type::Array,
            body: |_name, _intepreter, args| {
                let Variable::Array(array1) = args.get("array1")? else {
                    panic!()
                };
                let Variable::Array(array2) = args.get("array2")? else {
                    panic!()
                };
                let mut new_array = (*array1).clone();
                for element in array2.iter() {
                    new_array.push(element.clone());
                }
                Ok(Variable::Array(new_array.into()))
            },
        },
    );
    variables.add_native_function(
        "array_len",
        NativeFunction {
            params: Params {
                standard: params!("array": Type::Array),
                catch_rest: None,
            },
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Array(array) = args.get("array")? else {
                    panic!()
                };
                Ok(Variable::Float(array.len() as f64))
            },
        },
    );
    variables.add_native_function(
        "for_each",
        NativeFunction {
            params: Params {
                standard: params!("array": Type::Array, "function": Type::Function(Type::Any.into(), vec![Type::Any], false)),
                catch_rest: None
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
            params: Params{
                standard: params!("array": Type::Array, "function": Type::Function(Type::Float.into(), vec![Type::Any], false)),
                catch_rest: None
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
                        != Variable::Float(0.0)
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
                    "function": Type::Function(Type::Any.into(), vec![Type::Any, Type::Any], false)
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
    variables.add_native_function(
        "zip",
        NativeFunction {
            params: Params {
                standard: params!("array1": Type::Array, "array2": Type::Array),
                catch_rest: None,
            },
            return_type: Type::Array,
            body: |_name, _intepreter, args| {
                let Variable::Array(array1) = args.get("array1")? else {
                    panic!()
                };
                let Variable::Array(array2) = args.get("array2")? else {
                    panic!()
                };
                let new_array: Array = zip(array1.iter(), array2.iter())
                    .map(|(element1, element2)| {
                        Variable::Array(vec![element1.clone(), element2.clone()].into())
                    })
                    .collect();
                Ok(Variable::Array(new_array.into()))
            },
        },
    );
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
                    "function": Type::Function(Type::Any.into(), vec![Type::Any], false),
                    "n": Type::Float
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
                let Variable::Float(n) = args.get("n")? else {
                    panic!();
                };
                if !is_natural(n) {
                    return Err(Error::WrongType(
                        String::from("n"),
                        /*Fix it later String::from("natural")*/ Type::Float,
                    ));
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
                    "function": Type::Function(Type::Any.into(), vec![Type::Array], false),
                    "n": Type::Float
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
                let Variable::Float(n) = args.get("n")? else {
                    panic!();
                };
                if !is_natural(n) {
                    return Err(Error::WrongType(
                        String::from("n"),
                        /*Fix it later String::from("natural")*/ Type::Float,
                    ));
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

fn is_natural(f: f64) -> bool {
    f.fract() == 0.0 && f >= 0.0
}
