use std::io;
use crate::variable::*;
use crate::*;

pub fn add_io_functions(intepreter: &mut Intepreter){
    add_function!("print", intepreter, args, {
        let mut text = String::new();
        for arg in args {
            text+=&arg.to_string();
        }
        println!("{}", text);
        Ok(Variable::Null)
    });
    add_function!("cgetline", intepreter, _args, {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read user input");
        input = input.replace("\n", "");
        Ok(Variable::Text(input))
    });
}