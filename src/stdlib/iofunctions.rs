use crate::{
    function::{NativeFunction, Param, Params},
    interpreter::Interpreter,
    variable::*,
    Result,
};
use simplesl_macros::export_function;
use std::io;

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function]
    fn print(vars: &[Variable]) {
        let text = vars
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.to_string());
        println!("{text}");
    }

    #[export_function]
    fn cgetline() -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(input)
    }
}
