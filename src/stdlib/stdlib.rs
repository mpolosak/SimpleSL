use crate::function::{NativeFunction, Param};
use crate::{intepreter::VariableMap,params,variable::*};

pub fn add_std_functions(variables: &mut VariableMap){
    variables.add_native_function("import", NativeFunction{
        params: params!("path":"string"),
        body: |_name, intepreter, args|{
            let Variable::String(path) = args.get("path")? else {
                panic!()
            };
            intepreter.load_and_exec(&path)
        }
    });
    variables.add_native_function("add", NativeFunction{
        params: params!("a":"float", "b":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                panic!()
            };
            let Variable::Float(b) = args.get("b")? else {
                panic!()
            };
            Ok(Variable::Float(a+b))
        }
    });
    variables.add_native_function("subtract", NativeFunction{
        params: params!("a":"float", "b":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                panic!()
            };
            let Variable::Float(b) = args.get("b")? else {
                panic!()
            };
            Ok(Variable::Float(a-b))
        }
    });
    variables.add_native_function("multiply", NativeFunction{
        params: params!("a":"float", "b":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                panic!()
            };
            let Variable::Float(b) = args.get("b")? else {
                panic!()
            };
            Ok(Variable::Float(a*b))
        }
    });
    variables.add_native_function("divide", NativeFunction{
        params: params!("a":"float", "b":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                panic!()
            };
            let Variable::Float(b) = args.get("b")? else {
                panic!()
            };
            Ok(Variable::Float(a/b))
        }
    });
    variables.add_native_function("modulo", NativeFunction{
        params: params!("a":"float", "b":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                panic!()
            };
            let Variable::Float(b) = args.get("b")? else {
                panic!()
            };
            let divided = a/b;
            let result = a - b*divided.floor();
            Ok(Variable::Float(result))
        }
    });
    variables.add_native_function("or", NativeFunction{
        params: params!("a":"float", "b":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                panic!()
            };
            let Variable::Float(b) = args.get("b")? else {
                panic!()
            };
            Ok(Variable::Float(a.abs()+b.abs()))
        }
    });
    variables.add_native_function("not", NativeFunction{
        params: params!("a":"float"),
        body: |_name, _intepreter, args|{
            let Variable::Float(a) = args.get("a")? else {
                panic!()
            };
            Ok(Variable::Float(if a==0.0{1.0}else{0.0}))
        }
    });
    variables.add_native_function("equals", NativeFunction{
        params: params!("a":"any", "b":"any"),
        body: |_name, _intepreter, args|{
            let a = args.get("a")?;
            let b = args.get("b")?;
            Ok(Variable::Float(if a==b {1.0}  else {0.0}))
        }
    });
    variables.add_native_function("if", NativeFunction{
        params: params!("condition":"float", "function":"function"),
        body: |_name, intepreter, args|{
            let Variable::Float(condition) = args.get("condition")? else {
                panic!()
            };
            let Variable::Function(function) = args.get("function")? else {
                panic!()
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