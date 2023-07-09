use crate::{
    error::Error,
    function::{NativeFunction, Params},
    interpreter::VariableMap,
    variable::*,
    variable_type::Type,
};
use simplesl_macros::export_function;
use std::io;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function(catch_rest = true)]
    fn print(vars: &Array) {
        let text = vars
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.to_string());
        println!("{text}");
    }

    #[export_function]
    fn cgetline() -> Result<String, String> {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop();
                Ok(input)
            }
            Err(error) => Err(Error::from(error).to_string()),
        }
    }
}
