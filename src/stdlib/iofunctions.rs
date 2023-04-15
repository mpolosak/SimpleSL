use std::io;
use crate::function::{NativeFunction, Param};
use crate::{intepreter::VariableMap,variable::*,params};

pub fn add_io_functions(variables: &mut VariableMap){
    variables.add_native_function("print", NativeFunction{
        params: params!("vars":"..."),
        body: |_name, _intepreter, args|{
            let Variable::Array(args) = args.get("vars")? else {
                panic!();
            };
            let mut text = String::new();
            for arg in args.iter() {
                text+=&arg.to_string();
            }
            println!("{text}");
            Ok(Variable::Null)
        }
    });
    variables.add_native_function("cgetline", NativeFunction{
        params: params!(),
        body: |_name, _intepreter, _params|{
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input = input.replace('\n', "");
            Ok(Variable::String(input.into()))
        }
    });
}