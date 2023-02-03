use std::io;
use crate::error::Error;
use crate::function::{NativeFunction, Param};
use crate::intepreter::VariableMap;
use crate::variable::*;

pub fn add_io_functions(variables: &mut VariableMap){
    variables.add_native_function("print", NativeFunction{
        params: vec!(
            Param::new("vars", "..."),
        ),
        body: |_name, _intepreter, args|{
            let Variable::Array(args) = args.get("vars")? else {
                return Err(Error::SomethingStrange)
            };
            let mut text = String::new();
            for arg in args {
                text+=&arg.to_string();
            }
            println!("{}", text);
            Ok(Variable::Null)
        }
    });
    variables.add_native_function("cgetline", NativeFunction{
        params: Vec::new(),
        body: |_name, _intepreter, _params|{
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unable to read user input");
            input = input.replace("\n", "");
            Ok(Variable::Text(input))
        }
    });
}