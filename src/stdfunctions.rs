use crate::variable::*;
use crate::function::{Function, NativeFunction, Param};
use crate::intepreter::VariableMap;

pub fn add_std_functions(variables: &mut VariableMap){
    variables.add_native_function("import", NativeFunction{
        params: vec!(Param::new("path", "Text")),
        body: |name, intepreter, args|{
            let Variable::Text(path) = args.get("path")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            intepreter.load_and_exec(&path)
        }
    });
    variables.add_native_function("add", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Float(b) = args.get("b")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            Ok(Variable::Float(a+b))
        }
    });
    variables.add_native_function("subtract", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Float(b) = args.get("b")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            Ok(Variable::Float(a-b))
        }
    });
    variables.add_native_function("multiply", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Float(b) = args.get("b")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            Ok(Variable::Float(a*b))
        }
    });
    variables.add_native_function("divide", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Float(b) = args.get("b")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            Ok(Variable::Float(a/b))
        }
    });
    variables.add_native_function("modulo", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Float(b) = args.get("b")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let divided = a/b;
            let result = a - b*divided.floor();
            Ok(Variable::Float(result))
        }
    });
    variables.add_native_function("or", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Float(b) = args.get("b")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            Ok(Variable::Float(a.abs()+b.abs()))
        }
    });
    variables.add_native_function("not", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
        ),
        body: |name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            Ok(Variable::Float(if a==0.0{1.0}else{0.0}))
        }
    });
    variables.add_native_function("if", NativeFunction{
        params: vec!(
            Param::new("condition", "Float"),
            Param::new("function", "Function")
        ),
        body: |name, intepreter, args|{
            let Variable::Float(condition) = args.get("condition")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            let Variable::Function(function) = args.get("function")? else {
                return Err(format!("{name}: Something strange happend"))
            };
            if condition == 0.0 {
                Ok(Variable::Null)
            } else {
                function.exec(String::from("function"), intepreter, Array::new())
            }
        }
    });
    // add_function!("if", intepreter, args, (condition: Float, function: Function,){
    //     if condition == 0.0 { return Ok(Variable::Null)};
    //     function(intepreter, args)
    // });
    // add_function!("while", intepreter, args, {
    //     loop{
    //         let condition = get_var!("while", intepreter, args, 0, Float);
    //         if condition == 0.0 { break };
    //         let function = get_var!("while", intepreter, args, 1, Function);
    //         let function_args = if let Some(fargs) = args.get(2..) { fargs.to_vec() }
    //                               else { Array::new() };
    //         if let Err(error) = function(intepreter, function_args){
    //             return Err(error)
    //         }
    //     }
    //     return Ok(Variable::Null)
    // });
}