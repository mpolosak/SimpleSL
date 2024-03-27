use crate as simplesl;
use crate::{interpreter::Interpreter, join, variable::*};
use simplesl_macros::export_function;
use std::io;

/// Add input/output part of standard library to Interpreter
pub fn add_io(interpreter: &mut Interpreter) {
    #[export_function]
    fn print(var: Variable) {
        println!("{var}");
    }

    #[export_function]
    fn print_array(array: &[Variable], sep: &str) {
        println!("{}", join(array, sep));
    }

    #[export_function]
    fn cgetline() -> io::Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(input)
    }
}
