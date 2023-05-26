use crate::function::{Line, NativeFunction};
use crate::variable_type::Type;
use crate::{error::Error, parse::*, pest::Parser, stdlib::add_std_lib, variable::*};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    rc::Rc,
};
pub struct Intepreter {
    pub variables: VariableMap,
}

impl Intepreter {
    pub fn new() -> Intepreter {
        let mut variables = VariableMap::new();
        add_std_lib(&mut variables);
        Intepreter { variables }
    }

    pub fn exec(&mut self, input: String) -> Result<Variable, Error> {
        let parse = SimpleSLParser::parse(Rule::input, &input)?;
        let mut lines = Vec::<Line>::new();
        let mut local_variables = HashMap::<String, Type>::new();
        for line_pair in parse {
            if line_pair.as_rule() == Rule::EOI {
                break;
            }
            let line = Line::new(&self.variables, line_pair, &mut local_variables)?;
            lines.push(line);
        }
        let mut result = Variable::Null;
        for line in lines {
            result = line.exec_global(self)?;
        }
        Ok(result)
    }

    pub fn load_and_exec(&mut self, path: &str) -> Result<Variable, Error> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        self.exec(contents)
    }
}

#[derive(Debug)]
pub struct VariableMap {
    hash_map: HashMap<String, Variable>,
}

impl VariableMap {
    pub fn new() -> Self {
        VariableMap {
            hash_map: HashMap::new(),
        }
    }
    pub fn get(&self, name: &str) -> Result<Variable, Error> {
        match self.hash_map.get(name) {
            Some(variable) => Ok(variable.clone()),
            _ => Err(Error::VariableDoesntExist(String::from(name))),
        }
    }
    pub fn insert(&mut self, name: &str, variable: Variable) {
        self.hash_map.insert(String::from(name), variable);
    }
    pub fn add_native_function(&mut self, name: &str, function: NativeFunction) {
        self.insert(name, Variable::Function(Rc::new(function)))
    }
}
