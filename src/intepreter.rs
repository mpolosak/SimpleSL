use std::{collections::{HashMap, HashSet},fs::File,rc::Rc,io::{BufReader, BufRead}};
use crate::function::{NativeFunction, Line};
use crate::{parse::*,variable::*,error::Error,pest::Parser,stdlib::add_std_lib};
pub struct Intepreter{
    pub variables:  VariableMap
}

impl Intepreter{
    pub fn new() -> Intepreter{ 
        let variables = VariableMap::new();
        let mut intepreter = Intepreter{variables};
        add_std_lib(&mut intepreter.variables);
        intepreter
    }

    pub fn exec(&mut self, mut input: String) -> Result<Variable, Error>{
        input = input.trim().to_string();
        if input.is_empty() {
            return Ok(Variable::Null)
        }
        let parse = SimpleSLParser::parse(Rule::input, &input)?;
        let mut lines = Vec::<Line>::new();
        for line_pair in parse {
            if line_pair.as_rule() == Rule::EOI {
                break;
            }
            let line = Line::new(
                &self.variables, line_pair,
                &mut HashSet::new()
            )?;
            lines.push(line);
        }
        let mut result = Variable::Null;
        for line in lines {
            result = line.exec_global(self)?;
        }
        Ok(result)
    }

    pub fn load_and_exec(&mut self, path: &str) -> Result<Variable, Error>{
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        let mut result = Variable::Null;
        for line in buf_reader.lines() {
            let text = line?;
            result = self.exec(text)?;
        }
        Ok(result)
    }
}

pub struct VariableMap {
    hash_map: HashMap<String, Variable>
}

impl VariableMap{
    pub fn new() -> Self {
        VariableMap { hash_map: HashMap::new() }
    }
    pub fn get(&self, name: &str) -> Result<Variable, Error>{
        match self.hash_map.get(name) {
            Some(variable) => Ok(variable.clone()),
            _ => Err(Error::VariableDoesntExist(String::from(name))),
        }
    }
    pub fn insert(&mut self, name: &str, variable: Variable){
        self.hash_map.insert(String::from(name), variable);
    }
    pub fn add_native_function(&mut self, name: &str, function: NativeFunction){
        self.insert(name, Variable::Function(Rc::new(function)))
    }
}