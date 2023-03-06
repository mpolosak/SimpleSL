use std::io;
use std::rc::Rc;
use crate::error::Error;
use crate::function::{NativeFunction, Param};
use crate::intepreter::VariableMap;
use crate::variable::*;
use crate::params;

pub fn add_io_functions(variables: &mut VariableMap){
    variables.add_native_function("print", NativeFunction{
        params: params!("vars":"..."),
        body: |_name, _intepreter, args|{
            let Variable::Array(args) = args.get("vars")? else {
                return Err(Error::SomethingStrange)
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
            io::stdin().read_line(&mut input).expect("Unable to read user input");
            input = input.replace('\n', "");
            Ok(Variable::Text(Rc::new(input)))
        }
    });
}