use crate::instruction::{local_variable::LocalVariableMap, Exec, Instruction};
use crate::{
    error::Error, function::NativeFunction, parse::*, pest::Parser, stdlib::add_std_lib,
    variable::*,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    rc::Rc,
};

pub struct Interpreter {
    pub variables: VariableMap,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut variables = VariableMap::new();
        add_std_lib(&mut variables);
        Interpreter { variables }
    }

    pub fn exec(&mut self, input: &str) -> Result<Variable, Error> {
        let instructions = self.parse_input(input)?;
        let mut variables = VariableMap::new();
        let mut result = Variable::Void;
        for instruction in instructions {
            result = instruction.exec(self, &mut variables)?;
        }
        self.variables.extend(variables);
        Ok(result)
    }

    pub fn load_and_exec(&mut self, path: &str) -> Result<Variable, Error> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        self.exec(&contents)
    }

    fn parse_input(&self, input: &str) -> Result<Vec<Instruction>, Error> {
        let parse = SimpleSLParser::parse(Rule::input, input)?;
        let mut instructions = Vec::new();
        let mut local_variables = LocalVariableMap::new();
        for pair in parse {
            if pair.as_rule() == Rule::EOI {
                break;
            }
            let instruction = Instruction::new(pair, &self.variables, &mut local_variables)?;
            instructions.push(instruction);
        }
        Ok(instructions)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Default)]
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
    pub fn extend(&mut self, other: VariableMap) {
        self.hash_map.extend(other.hash_map)
    }
}
