use crate::instruction::{local_variable::LocalVariables, Exec, Instruction};
use crate::{parse::*, pest::Parser, stdlib::add_std_lib, variable::*, Error, Result};
use std::fs::read_to_string;
use std::{collections::HashMap, rc::Rc};

#[derive(Debug)]
#[must_use]
pub struct Interpreter<'a> {
    variables: VariableMap,
    lower_layer: Option<&'a Self>,
}

type VariableMap = HashMap<Rc<str>, Variable>;

impl<'a> Interpreter<'a> {
    /// Constructs a new Interpreter with simplesl stdlib
    pub fn with_stdlib() -> Self {
        let mut interpreter = Self::without_stdlib();
        add_std_lib(&mut interpreter);
        interpreter
    }

    /// Constructs a new Interpreter without stdlib
    pub fn without_stdlib() -> Self {
        Self {
            variables: VariableMap::new(),
            lower_layer: None,
        }
    }

    /// Executes all instructions in order  
    /// If any of calls result in error stop executing and returns that error  
    /// If all instructions get executed successfully returns result of last instruction
    /// or Variable::Void if instructions is empty
    pub fn exec(&mut self, instructions: &[Instruction]) -> Result<Variable> {
        instructions
            .iter()
            .map(|instruction| instruction.exec(self))
            .last()
            .unwrap_or(Ok(Variable::Void))
    }

    pub fn load(
        &self,
        path: &str,
        local_variables: &mut LocalVariables,
    ) -> Result<Box<[Instruction]>> {
        let contents = read_to_string(path)?;
        self.parse_input(&contents, local_variables)
    }

    pub fn parse_input(
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

    pub fn create_layer(&'a self) -> Self {
        Self {
            variables: VariableMap::new(),
            lower_layer: Some(self),
        }
    }
}
