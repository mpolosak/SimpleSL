use crate::instruction::Exec;
use crate::instruction::{ExecStop, InstructionWithStr};
use crate::{stdlib, variable::*};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
#[must_use]
pub struct Interpreter {
    variables: Vec<VariableMap>,
}

type VariableMap = HashMap<Arc<str>, Variable>;

impl Interpreter {
    /// Constructs a new Interpreter with simplesl stdlib
    pub fn with_stdlib() -> Self {
        let mut interpreter = Self::without_stdlib();
        stdlib::add_all(&mut interpreter);
        interpreter
    }

    /// Constructs a new Interpreter without stdlib
    pub fn without_stdlib() -> Self {
        Self {
            variables: vec![VariableMap::new()],
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
        for map in self.variables.iter().rev() {
            let var = map.get(name);
            if var.is_some() {
                return var;
            }
        }
        None
    }
    pub fn insert(&mut self, name: Arc<str>, variable: Variable) {
        self.variables.last_mut().unwrap().insert(name, variable);
    }

    pub(crate) fn push_layer(&mut self) {
        self.variables.push(VariableMap::new());
    }
    pub(crate) fn pop_layer(&mut self) {
        if self.variables.len() < 2 {
            panic!("Interpreter.pop_layer() called on Interpreter containing less than two layers");
        }
        self.variables.pop();
    }
}
