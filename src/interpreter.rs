use crate::instruction::{local_variable::LocalVariables, Exec, Instruction};
use crate::{parse::*, pest::Parser, stdlib::add_std_lib, variable::*, Error, Result};
use std::fs::read_to_string;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct Interpreter<'a> {
    variables: VariableMap,
    lower_layer: Option<&'a Self>,
}

type VariableMap = HashMap<Rc<str>, Variable>;

impl<'a> Interpreter<'a> {
    #[must_use]
    pub fn new() -> Self {
        let mut interpreter = Self {
            variables: VariableMap::new(),
            lower_layer: None,
        };
        add_std_lib(&mut interpreter);
        interpreter
    }

    pub fn exec(&mut self, instructions: &[Instruction]) -> Result<Variable> {
        instructions
            .iter()
            .map(|instruction| instruction.exec(self))
            .last()
            .unwrap_or(Ok(Variable::Void))
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
        let contents = read_to_string(path)?;
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
            .map(|pair| Instruction::new(pair, self, local_variables))
            .collect::<Result<_>>()?;
        Ok(instructions)
    }
    pub fn get_variable(&self, name: &str) -> Result<Variable> {
        self.variables
            .get(name)
            .or_else(|| self.lower_layer?.variables.get(name))
            .map(Clone::clone)
            .ok_or_else(|| Error::VariableDoesntExist(name.into()))
    }
    pub fn insert(&mut self, name: Rc<str>, variable: Variable) {
        self.variables.insert(name, variable);
    }

    #[must_use]
    pub fn create_layer(&'a self) -> Self {
        Self {
            variables: VariableMap::new(),
            lower_layer: Some(self),
        }
    }
}

impl Default for Interpreter<'_> {
    fn default() -> Self {
        Self::new()
    }
}
