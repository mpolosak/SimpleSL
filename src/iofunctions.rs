use std::io;
use crate::intepreter::*;
use crate::params::*;
use crate::*;

pub fn add_io_functions(variables: &mut VariableMap){
    add_function!("print", variables, params, {
        let mut text = String::new();
        for param in params {
            match param {
                Param::Float(value) => text+=&value.to_string(),
                Param::Text(value) => text+=&value,
                Param::Variable(name) => match variables.get(&name){
                    Some(Variable::Float(value)) => text+=&value.to_string(),
                    Some(Variable::Text(value)) => text+=&value,
                    Some(Variable::Function(_)) => text+="Function",
                    Some(Variable::Null)=>text+="Null",
                    None => return Err(format!("Variable {} doesn't exist", name))
                }
            };
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