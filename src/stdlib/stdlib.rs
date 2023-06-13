use crate::function::{NativeFunction, Param, Params};
use crate::variable_type::Type;
use crate::{intepreter::VariableMap, params, variable::*};
extern crate simplesl_macros;
use simplesl_macros::export_function_of_two_floats;

pub fn add_std_functions(variables: &mut VariableMap) {
    variables.add_native_function(
        "import",
        NativeFunction {
            params: Params {
                standard: params!("path": Type::String),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::String(path) = args.get("path")? else {
                    panic!()
                };
                intepreter.load_and_exec(&path)
            },
        },
    );

    #[export_function_of_two_floats]
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    #[export_function_of_two_floats]
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    #[export_function_of_two_floats]
    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    #[export_function_of_two_floats]
    fn divide(a: f64, b: f64) -> f64 {
        a / b
    }

    #[export_function_of_two_floats]
    fn modulo(a: f64, b: f64) -> f64 {
        let divided = a / b;
        a - b * divided.floor()
    }

    #[export_function_of_two_floats]
    fn or(a: f64, b: f64) -> f64 {
        a.abs() + b.abs()
    }

    variables.add_native_function(
        "not",
        NativeFunction {
            params: Params {
                standard: params!("a": Type::Float),
                catch_rest: None,
            },
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Float(a) = args.get("a")? else {
                    panic!()
                };
                Ok(Variable::Float(if a == 0.0 { 1.0 } else { 0.0 }))
            },
        },
    );
    variables.add_native_function(
        "equals",
        NativeFunction {
            params: Params {
                standard: params!("a": Type::Any, "b": Type::Any),
                catch_rest: None,
            },
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let a = args.get("a")?;
                let b = args.get("b")?;
                Ok(Variable::Float(if a == b { 1.0 } else { 0.0 }))
            },
        },
    );
    variables.add_native_function(
        "if",
        NativeFunction {
            params: Params {
                standard: params!("condition": Type::Float, "function": Type::Function(Type::Any.into(), vec![], false)),
                catch_rest: None
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Float(condition) = args.get("condition")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                if condition == 0.0 {
                    Ok(Variable::Null)
                } else {
                    function.exec("function", intepreter, Array::new())
                }
            },
        },
    );
    variables.add_native_function(
        "if_else",
        NativeFunction {
            params: Params {
                standard: params!(
                    "condition": Type::Float,
                    "function": Type::Function(Type::Any.into(), vec![], false),
                    "else": Type::Function(Type::Any.into(), vec![], false)
                ),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Float(condition) = args.get("condition")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                let Variable::Function(else_function) = args.get("else")? else {
                    panic!()
                };
                if condition == 0.0 {
                    else_function.exec("else", intepreter, Array::new())
                } else {
                    function.exec("function", intepreter, Array::new())
                }
            },
        },
    );
}
