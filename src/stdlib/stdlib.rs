use crate::function::{NativeFunction, Param};
use crate::variable_type::Type;
use crate::{intepreter::VariableMap, params, variable::*};

pub fn add_std_functions(variables: &mut VariableMap) {
    variables.add_native_function(
        "import",
        NativeFunction {
            params: params!("path": Type::String),
            return_type: Type::Any,
            body: |_name, intepreter, args| {
                let Variable::String(path) = args.get("path")? else {
                    panic!()
                };
                intepreter.load_and_exec(&path)
            },
        },
    );
    variables.add_native_function(
        "add",
        NativeFunction {
            params: params!("a": Type::Float, "b": Type::Float),
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Float(a) = args.get("a")? else {
                    panic!()
                };
                let Variable::Float(b) = args.get("b")? else {
                    panic!()
                };
                Ok(Variable::Float(a + b))
            },
        },
    );
    variables.add_native_function(
        "subtract",
        NativeFunction {
            params: params!("a": Type::Float, "b": Type::Float),
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Float(a) = args.get("a")? else {
                    panic!()
                };
                let Variable::Float(b) = args.get("b")? else {
                    panic!()
                };
                Ok(Variable::Float(a - b))
            },
        },
    );
    variables.add_native_function(
        "multiply",
        NativeFunction {
            params: params!("a": Type::Float, "b": Type::Float),
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Float(a) = args.get("a")? else {
                    panic!()
                };
                let Variable::Float(b) = args.get("b")? else {
                    panic!()
                };
                Ok(Variable::Float(a * b))
            },
        },
    );
    variables.add_native_function(
        "divide",
        NativeFunction {
            params: params!("a": Type::Float, "b": Type::Float),
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Float(a) = args.get("a")? else {
                    panic!()
                };
                let Variable::Float(b) = args.get("b")? else {
                    panic!()
                };
                Ok(Variable::Float(a / b))
            },
        },
    );
    variables.add_native_function(
        "modulo",
        NativeFunction {
            params: params!("a": Type::Float, "b": Type::Float),
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Float(a) = args.get("a")? else {
                    panic!()
                };
                let Variable::Float(b) = args.get("b")? else {
                    panic!()
                };
                let divided = a / b;
                let result = a - b * divided.floor();
                Ok(Variable::Float(result))
            },
        },
    );
    variables.add_native_function(
        "or",
        NativeFunction {
            params: params!("a": Type::Float, "b": Type::Float),
            return_type: Type::Float,
            body: |_name, _intepreter, args| {
                let Variable::Float(a) = args.get("a")? else {
                    panic!()
                };
                let Variable::Float(b) = args.get("b")? else {
                    panic!()
                };
                Ok(Variable::Float(a.abs() + b.abs()))
            },
        },
    );
    variables.add_native_function(
        "not",
        NativeFunction {
            params: params!("a": Type::Float),
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
            params: params!("a": Type::Any, "b": Type::Any),
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
            params: params!("condition": Type::Float, "function": Type::Function),
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
            params: params!(
                "condition": Type::Float,
                "function": Type::Function,
                "else": Type::Function
            ),
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
