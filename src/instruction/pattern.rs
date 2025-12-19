use crate::{
    function::Param, instruction::local_variable::{LocalVariable, LocalVariables}, variable::{Type, Variable}, Interpreter
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub ident: Arc<str>,
    pub var_type: Type,
}

impl Pattern {
    pub fn new_ident_pattern(ident: Arc<str>, var_type: Type) -> Self {
        Pattern { ident, var_type }
    }

    pub fn create_instruction(pair: Pair<Rule>, _local_variables: &mut LocalVariables, var_type: &Type) -> Self {
        let mut inner = pair.into_inner();
        let ident = inner.next().unwrap().as_str().into();
        let var_type = inner.next().map(Type::from).unwrap_or_else(|| var_type.clone());
        Self { ident, var_type }
    }

    pub fn is_matched(&self, var_type: &Type) -> bool {
        var_type.matches(&self.var_type)
    }

    pub fn insert_local_variables(&self, local_variables: &mut LocalVariables) {
        local_variables.insert(self.ident.clone(), LocalVariable::Other(self.var_type.clone()));
    }

    pub fn insert_variables(&self, interpreter: &mut Interpreter, variable: Variable) {
        interpreter.insert(self.ident.clone(), variable);
    }
}

impl From<Param> for Pattern {
    fn from(value: Param) -> Self {
        Self { ident: value.name, var_type: value.var_type}
    }
}
