use crate::error::Error;
use crate::function::{NativeFunction, Params};
use crate::params;
use crate::variable_type::Type;
use crate::{intepreter::VariableMap, variable::*};
use simplesl_macros::export_function;
use std::io;

pub fn add_io_functions(variables: &mut VariableMap) {
    variables.add_native_function(
        "print",
        NativeFunction {
            params: Params {
                standard: Vec::new(),
                catch_rest: Some(String::from("vars")),
            },
            return_type: Type::Null,
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

    #[export_function]
    fn cgetline() -> Result<Variable, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(Variable::String(input.into()))
    }
}
