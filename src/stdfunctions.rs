use crate::variable::*;
use crate::function::{Function, NativeFunction, Param};
use crate::intepreter::Intepreter;

pub fn add_std_functions(intepreter: &mut Intepreter){
    intepreter.add_function("import", NativeFunction{
        params: vec!(Param::new("path", "Text")),
        body: |_name, intepreter, args|{
            let Some(Variable::Text(path)) = args.get(
                &String::from("path")) else {
                return Err(String::from("Something strange happend"))
            };
            intepreter.load_and_exec(path)
        }
    });
    intepreter.add_function("add", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Float(a)) = args.get(
                &String::from("a")) else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Float(b)) = args.get(
                &String::from("b")) else {
                return Err(String::from("Something strange happend"))
            };
            Ok(Variable::Float(a+b))
        }
    });
    intepreter.add_function("subtract", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Float(a)) = args.get(
                &String::from("a")) else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Float(b)) = args.get(
                &String::from("b")) else {
                return Err(String::from("Something strange happend"))
            };
            Ok(Variable::Float(a-b))
        }
    });
    intepreter.add_function("multiply", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Float(a)) = args.get(
                &String::from("a")) else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Float(b)) = args.get(
                &String::from("b")) else {
                return Err(String::from("Something strange happend"))
            };
            Ok(Variable::Float(a*b))
        }
    });
    intepreter.add_function("divide", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Float(a)) = args.get(
                &String::from("a")) else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Float(b)) = args.get(
                &String::from("b")) else {
                return Err(String::from("Something strange happend"))
            };
            Ok(Variable::Float(a/b))
        }
    });
    intepreter.add_function("modulo", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Float(a)) = args.get(
                &String::from("a")) else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Float(b)) = args.get(
                &String::from("b")) else {
                return Err(String::from("Something strange happend"))
            };
            let divided = a/b;
            let result = a - b*divided.floor();
            Ok(Variable::Float(result))
        }
    });
    intepreter.add_function("or", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
            Param::new("b", "Float")
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Float(a)) = args.get(
                &String::from("a")) else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Float(b)) = args.get(
                &String::from("b")) else {
                return Err(String::from("Something strange happend"))
            };
            Ok(Variable::Float(a.abs()+b.abs()))
        }
    });
    intepreter.add_function("not", NativeFunction{
        params: vec!(
            Param::new("a", "Float"),
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Float(a)) = args.get(
                &String::from("a")) else {
                return Err(String::from("Something strange happend"))
            };
            Ok(Variable::Float(if *a==0.0{1.0}else{0.0}))
        }
    });
    intepreter.add_function("if", NativeFunction{
        params: vec!(
            Param::new("condition", "Float"),
            Param::new("function", "Function")
        ),
        body: |_name, intepreter, args|{
            let Some(Variable::Float(condition)) = args.get(
                &String::from("condition")) else {
                return Err(String::from("Something strange happend"))
            };
            let Some(Variable::Function(function)) = args.get(
                &String::from("function")) else {
                return Err(String::from("Something strange happend"))
            };
            if *condition == 0.0 {
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