use crate::{
    error::Error,
    function::{NativeFunction, Params},
    interpreter::Interpreter,
    variable::*,
};
use simplesl_macros::export_function;
use std::io;

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function(catch_rest = true)]
    fn print(vars: &[Variable]) {
        let text = vars
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.to_string());
        println!("{text}");
    }

    #[export_function]
    fn cgetline() -> Result<String, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(input)
    }
}
