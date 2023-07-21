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
        for instruction in instructions.iter() {
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

    fn parse_input(&self, input: &str) -> Result<Box<[Instruction]>, Error> {
        let parse = SimpleSLParser::parse(Rule::input, input)?;
        let mut local_variables = LocalVariableMap::new();
        let instructions = parse
            .take_while(|pair| pair.as_rule() != Rule::EOI)
            .map(|pair| Instruction::new(pair, &self.variables, &mut local_variables))
            .collect::<Result<_, _>>()?;
        Ok(instructions)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

pub type VariableMap = HashMap<String, Variable>;

pub trait VariableMapTrait {
    fn try_get(&self, name: &str) -> Result<Variable, Error>;
    fn add_native_function(&mut self, name: String, function: NativeFunction);
}

impl VariableMapTrait for VariableMap {
    fn try_get(&self, name: &str) -> Result<Variable, Error> {
        match self.get(name) {
            Some(variable) => Ok(variable.clone()),
            _ => Err(Error::VariableDoesntExist(String::from(name))),
        }
    }
    fn add_native_function(&mut self, name: String, function: NativeFunction) {
        self.insert(name, Variable::Function(Rc::new(function)));
    }
}
