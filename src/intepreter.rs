use crate::parse::*;
use crate::stdfunctions::*;
use crate::iofunctions::*;
use crate::variable::*;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Intepreter{
    pub variables:  VariableMap
}

impl Intepreter{
    pub fn new() -> Intepreter{ 
        let variables = VariableMap::new();
        let mut intepreter = Intepreter{variables};
        add_io_functions(&mut intepreter);
        add_std_functions(&mut intepreter);
        intepreter
    }

    pub fn exec(&mut self, mut line: String) -> Result<Variable, String>{
        let result_var = get_result_var(&mut line)?;

        line = line.trim().to_string();
        let result = get_var(&mut line, self)?;
        if !line.is_empty() {
            return Err(String::from("Syntax error"))
        }

        if let Some(var) = result_var {
            self.variables.insert(var, result);
            Ok(Variable::Null)
        } else {
            Ok(result)
        }
    }

    pub fn load_and_exec(&mut self, path: String) -> Result<Variable, String>{
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        let mut result = Variable::Null;
        for line in buf_reader.lines() {
            let text = line.unwrap();
            result = self.exec(text)?;
        }
        Ok(result)
    }

    pub fn get_variable(&self, name: String) -> Result<Variable, String>{
        match self.variables.get(&name) {
            Some(variable) => Ok(variable.clone()),
            _ => Err(format!("Variable {} doesn't exist", name)),
        }
    }
}