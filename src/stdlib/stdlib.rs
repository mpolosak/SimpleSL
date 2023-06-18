use crate::function::{NativeFunction, Param, Params};
use crate::variable_type::Type;
use crate::{intepreter::VariableMap, params, variable::*};
extern crate simplesl_macros;
use simplesl_macros::export_math_function;

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

    #[export_math_function]
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    #[export_math_function]
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }

    #[export_math_function]
    fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }

    #[export_math_function]
    fn divide(a: f64, b: f64) -> f64 {
        a / b
    }

    #[export_math_function]
    fn modulo(a: f64, b: f64) -> f64 {
        let divided = a / b;
        a - b * divided.floor()
    }

    #[export_math_function]
    fn or(a: f64, b: f64) -> f64 {
        a.abs() + b.abs()
    }

    #[export_math_function]
    fn not(a: f64) -> f64 {
        if a == 0.0 {
            1.0
        } else {
            0.0
        }
    }

    variables.add_native_function(
        "equals",
        NativeFunction {
            params: Params {
                standard: params!("a": Type::Any, "b": Type::Any),
                catch_rest: None,
            },
            return_type: Type::Int,
            body: |_name, _intepreter, args| {
                let a = args.get("a")?;
                let b = args.get("b")?;
                Ok(Variable::Int(if a == b { 1 } else { 0 }))
            },
        },
    );
    variables.add_native_function(
        "if",
        NativeFunction {
            params: Params {
                standard: params!(
                    "condition": Type::Int,
                    "function": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![],
                        catch_rest: false
                    }
                ),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Int(condition) = args.get("condition")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                if condition == 0 {
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
                    "condition": Type::Int,
                    "function": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![],
                        catch_rest: false
                    },
                    "else": Type::Function{
                        return_type: Type::Any.into(),
                        params: vec![],
                        catch_rest: false
                    }
                ),
                catch_rest: None,
            },
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::Int(condition) = args.get("condition")? else {
                    panic!()
                };
                let Variable::Function(function) = args.get("function")? else {
                    panic!()
                };
                let Variable::Function(else_function) = args.get("else")? else {
                    panic!()
                };
                if condition == 0 {
                    else_function.exec("else", intepreter, Array::new())
                } else {
                    function.exec("function", intepreter, Array::new())
                }
            },
        },
    );
}
