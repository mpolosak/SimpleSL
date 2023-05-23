use crate::function::{NativeFunction, Param};
use crate::{intepreter::VariableMap, params, variable::*};
use std::io;

pub fn add_io_functions(variables: &mut VariableMap) {
    variables.add_native_function(
        "print",
        NativeFunction {
            params: vec![Param::CatchRest(String::from("vars"))],
            body: |_name, _intepreter, args| {
                let Variable::Array(args) = args.get("vars")? else {
                    panic!();
                };
                let text = args
                    .iter()
                    .fold(String::new(), |acc, arg| acc + &arg.to_string());
                println!("{text}");
                Ok(Variable::Null)
            },
        },
    );
    variables.add_native_function(
        "cgetline",
        NativeFunction {
            params: params!(),
            body: |_name, _intepreter, _params| {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                input = input.replace('\n', "");
                Ok(Variable::String(input.into()))
            },
        },
    );
}
