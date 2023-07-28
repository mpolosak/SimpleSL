use crate::instruction::{local_variable::LocalVariables, Exec, Instruction};
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
    variables: Vec<VariableMap>,
}

type VariableMap = HashMap<Rc<str>, Variable>;

impl Interpreter {
    pub fn new() -> Interpreter {
        let variables = VariableMap::new();
        let mut result = Interpreter {
            variables: vec![variables],
        };
        add_std_lib(&mut result);
        result
    }

    pub fn exec(&mut self, input: &str) -> Result<Variable, Error> {
        let instructions = self.parse_input(input)?;
        let mut result = Variable::Void;
        for instruction in instructions.iter() {
            result = instruction.exec(self)?;
        }
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
        let mut local_variables = LocalVariables::new();
        let instructions = parse
            .take_while(|pair| pair.as_rule() != Rule::EOI)
            .map(|pair| Instruction::new(pair, self, &mut local_variables))
            .collect::<Result<_, _>>()?;
        Ok(instructions)
    }
    pub fn get_variable(&self, name: &str) -> Result<Variable, Error> {
        for layer in self.variables.iter().rev() {
            if let Some(variable) = layer.get(name) {
                return Ok(variable.clone());
            }
        }
        Err(Error::VariableDoesntExist(name.to_owned()))
    }
    pub fn insert(&mut self, name: Rc<str>, variable: Variable) {
        self.variables.last_mut().unwrap().insert(name, variable);
    }
    pub fn insert_native_function(&mut self, name: Rc<str>, function: NativeFunction) {
        self.variables
            .last_mut()
            .unwrap()
            .insert(name, Variable::Function(Rc::new(function)));
    }
    pub fn add_layer(&mut self) {
        self.variables.push(VariableMap::new())
    }
    pub fn remove_layer(&mut self) {
        if self.variables.len() > 1 {
            self.variables.pop();
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
