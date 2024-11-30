use crate::instruction::{Exec, ExecResult, Instruction};
use crate::instruction::{ExecStop, InstructionWithStr};
use crate::{stdlib, variable::*};
use match_any::match_any;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
#[must_use]
pub struct Interpreter {
    variables: Vec<VariableMap>,
    result: Option<Variable>,
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
            result: None,
        }
    }

    pub(crate) fn exec(&mut self, instruction: &Instruction) -> ExecResult {
        match_any! { instruction,
            Instruction::Variable(var) => {
                self.result = Some(var.clone());
                Ok(var.clone())
            },
            Instruction::LocalVariable(ident, _) => {
                let result = self
                .get_variable(&ident)
                .cloned()
                .unwrap_or_else(|| panic!("Tried to get variable {ident} that doest exist"));
                self.result = Some(result.clone());
                Ok(result)
            },
            Instruction::Function(ins) | Instruction::Array(ins) | Instruction::ArrayRepeat(ins)
            | Instruction::DestructTuple(ins) | Instruction::Tuple(ins)
            | Instruction::BinOperation(ins) | Instruction::For(ins) | Instruction::IfElse(ins)
            | Instruction::Loop(ins) | Instruction::Match(ins) | Instruction::Mut(ins)
            | Instruction::Reduce(ins) | Instruction::SetIfElse(ins) | Instruction::TypeFilter(ins)
            | Instruction::UnaryOperation(ins) => {
                let result = ins.exec(self);
                if let Ok(var) = &result {
                    self.result = Some(var.clone())
                }
                result
            },
            Instruction::Break => Err(ExecStop::Break),
            Instruction::Continue => Err(ExecStop::Continue),
            Instruction::EnterScope => {
                self.push_layer();
                Ok(Variable::Void)
            },
            Instruction::ExitScope => {
                self.pop_layer();
                Ok(Variable::Void)
            },
            Instruction::Return => {
                Err(ExecStop::Return)
            },
            Instruction::Set(ident) => {
                self.insert(ident.clone(), self.result().unwrap().clone());
                Ok(Variable::Void)
            },
            Instruction::Call => {
                let result = self.result().unwrap().clone().as_function().unwrap().exec(self)?;
                Ok(result)
            }
        }
    }

    pub(crate) fn exec_all(
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

    pub(crate) fn result(&self) -> Option<&Variable> {
        self.result.as_ref()
    }
}
