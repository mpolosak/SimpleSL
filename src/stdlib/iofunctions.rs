use crate::error::Error;
use crate::function::{NativeFunction, Params};
use crate::variable_type::Type;
use crate::{interpreter::VariableMap, variable::*};
use simplesl_macros::export_function;
use std::io;
use std::rc::Rc;

pub fn add_functions(variables: &mut VariableMap) {
    #[export_function(catch_rest = true)]
    fn print(vars: &Array) {
        let text = vars
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.to_string());
        println!("{text}");
    }

    #[export_function]
    fn cgetline() -> Result<Rc<str>, Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input = input.replace('\n', "");
        Ok(input.into())
    }
}
