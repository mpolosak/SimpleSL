use std::io;
use crate::params::*;
use crate::variable::*;
use crate::*;

pub fn add_io_functions(variables: &mut VariableMap){
    add_function!("print", variables, params, {
        let mut text = String::new();
        for param in params {
            text+=&get_var!(variables, param).to_string();
        }
        println!("{}", text);
        Ok(Variable::Null)
    });
    add_function!("cgetline", variables, _params, {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read user input");
        input = input.replace("\n", "");
        Ok(Variable::Text(input))
    });
}