use std::io;
use crate::function::{NativeFunction, Param};
use crate::variable::*;
use crate::*;

pub fn add_io_functions(intepreter: &mut Intepreter){
    intepreter.add_function("print", NativeFunction{
        params: vec!(
            Param::new("vars", "..."),
        ),
        body: |_name, _intepreter, args|{
            let Some(Variable::Array(args)) = args.get(
                &String::from("vars")) else {
                return Err(String::from("Something strange happend"))
            };
            let mut text = String::new();
            for arg in args {
                text+=&arg.to_string();
            }
            println!("{}", text);
            Ok(Variable::Null)
        }
    });
    intepreter.add_function("cgetline", NativeFunction{
        params: Vec::new(),
        body: |_name, _intepreter, _params|{
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unable to read user input");
            input = input.replace("\n", "");
            Ok(Variable::Text(input))
        }
    });
}