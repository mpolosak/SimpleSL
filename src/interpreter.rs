use crate::instruction::{Exec, ExecStop, InstructionWithStr};
use crate::{stdlib, variable::*};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
#[must_use]
pub struct Interpreter<'a> {
    variables: VariableMap,
    lower_layer: Option<&'a Self>,
}

pub type VariableMap = HashMap<Arc<str>, Variable>;

impl<'a> Interpreter<'a> {
    /// Constructs a new Interpreter with simplesl stdlib
    pub fn with_stdlib() -> Self {
        let mut interpreter = Self::without_stdlib();
        interpreter.insert("std".into(), stdlib::stdlib.clone());
        interpreter
    }

    /// Constructs a new Interpreter without stdlib
    pub fn without_stdlib() -> Self {
        Self {
            variables: VariableMap::new(),
            lower_layer: None,
        }
    }

    pub(crate) fn exec(
        &mut self,
        instructions: &[InstructionWithStr],
    ) -> Result<Arc<[Variable]>, ExecStop> {
        instructions
            .iter()
            .map(|instruction| instruction.exec(self))
            .collect()
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables
            .get(name)
            .or_else(|| self.lower_layer?.get_variable(name))
    }
    pub fn insert(&mut self, name: Arc<str>, variable: Variable) {
        self.variables.insert(name, variable);
    }

    pub fn create_layer(&'a self) -> Self {
        Self {
            variables: VariableMap::new(),
            lower_layer: Some(self),
        }
    }
}
