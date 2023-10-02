use crate::instruction::{local_variable::LocalVariables, Exec, Instruction};
use crate::{parse::*, pest::Parser, stdlib, variable::*, Result};
use std::{collections::HashMap, fs, rc::Rc};

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
        stdlib::add_all(&mut interpreter);
        interpreter
    }

    /// Constructs a new Interpreter without stdlib
    pub fn without_stdlib() -> Self {
        Self {
            variables: VariableMap::new(),
            lower_layer: None,
        }
    }

    pub(crate) fn exec(&mut self, instructions: &[Instruction]) -> Result<Variable> {
        instructions
            .iter()
            .map(|instruction| instruction.exec(self))
            .last()
            .unwrap_or(Ok(Variable::Void))
    }

    pub(crate) fn load(
        &self,
        path: &str,
        local_variables: &mut LocalVariables,
    ) -> Result<Rc<[Instruction]>> {
        let contents = fs::read_to_string(path)?;
        self.parse_input(&contents, local_variables)
    }

    fn parse_input(
        &self,
        input: &str,
        local_variables: &mut LocalVariables,
    ) -> Result<Rc<[Instruction]>> {
        let parse = SimpleSLParser::parse(Rule::input, input)?;
        let mut instructions = parse
            .map(|pair| Instruction::new(pair, self, local_variables))
            .collect::<Result<Vec<Instruction>>>()?;
        let Some(last) = instructions.pop() else {
            return Ok(Rc::from([]));
        };
        instructions.retain(|instruction| !matches!(instruction, Instruction::Variable(_)));
        instructions.push(last);
        Ok(instructions.into())
    }
    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables
            .get(name)
            .or_else(|| self.lower_layer?.variables.get(name))
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
