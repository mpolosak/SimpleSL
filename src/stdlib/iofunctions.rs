use crate::{function, interpreter::Interpreter, join, variable::*, Result};
use simplesl_macros::export_function;
use std::io;

pub fn add_functions(interpreter: &mut Interpreter) {
    #[export_function]
    fn print(var: Variable) {
        println!("{var}");
    }

    #[export_function]
    fn print_array(array: &[Variable], sep: &str) {
        println!("{}", join(array, sep));
    }

    #[export_function]
    fn cgetline() -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(input)
    }
}
