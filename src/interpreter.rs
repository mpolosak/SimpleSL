use crate::instruction::{local_variable::LocalVariables, Exec, Instruction};
use crate::{parse::*, pest::Parser, stdlib::add_std_lib, variable::*, Error, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    rc::Rc,
};

pub struct Interpreter<'a> {
    variables: VariableMap,
    lower_layer: Option<&'a Self>,
}

type VariableMap = HashMap<Rc<str>, Variable>;

impl<'a> Interpreter<'a> {
    #[must_use]
    pub fn new() -> Self {
        let variables = VariableMap::new();
        let mut result = Self {
            variables,
            lower_layer: None,
        };
        add_std_lib(&mut result);
        result
    }

    pub fn exec(&mut self, instructions: &[Instruction]) -> Result<Variable> {
        let mut result = Variable::Void;
        for instruction in instructions.iter() {
            result = instruction.exec(self)?;
        }
        Ok(result)
    }

    pub fn parse_and_exec(&mut self, input: &str) -> Result<Variable> {
        let instructions = self.parse_input(input, &mut LocalVariables::new())?;
        self.exec(&instructions)
    }

    pub fn load(
        &self,
        path: &str,
        local_variables: &mut LocalVariables,
    ) -> Result<Box<[Instruction]>> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        self.parse_input(&contents, local_variables)
    }

    pub fn load_and_exec(&mut self, path: &str) -> Result<Variable> {
        let instructions = self.load(path, &mut LocalVariables::new())?;
        self.exec(&instructions)
    }

    fn parse_input(
        &self,
        input: &str,
        local_variables: &mut LocalVariables,
    ) -> Result<Box<[Instruction]>> {
        let parse = SimpleSLParser::parse(Rule::input, input)?;
        let instructions = parse
            .take_while(|pair| pair.as_rule() != Rule::EOI)
            .map(|pair| Instruction::new(pair, self, local_variables))
            .collect::<Result<_>>()?;
        Ok(instructions)
    }
    pub fn get_variable(&self, name: &str) -> Result<Variable> {
        if let Some(variable) = self.variables.get(name) {
            Ok(variable.clone())
        } else if let Some(layer) = self.lower_layer {
            layer.get_variable(name)
        } else {
            Err(Error::VariableDoesntExist(name.into()))
        }
    }
    pub fn insert(&mut self, name: Rc<str>, variable: Variable) {
        self.variables.insert(name, variable);
    }

    #[must_use]
    pub fn create_layer(&'a self) -> Box<Self> {
        Self {
            variables: VariableMap::new(),
            lower_layer: Some(self),
        }
        .into()
    }
}

impl Default for Interpreter<'_> {
    fn default() -> Self {
        Self::new()
    }
}
