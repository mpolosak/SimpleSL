use super::{function::Function, Instruction};
use crate::{
    function::{Param, Params},
    variable::{function_type::FunctionType, GetReturnType, GetType, Type, Variable},
};
use std::{collections::HashMap, rc::Rc};

pub type LocalVariableMap = HashMap<Rc<str>, LocalVariable>;
pub struct LocalVariables {
    variables: Vec<LocalVariableMap>,
}

impl LocalVariables {
    pub fn new() -> Self {
        Self {
            variables: vec![LocalVariableMap::new()],
        }
    }
    pub fn insert(&mut self, name: Rc<str>, variable: LocalVariable) {
        self.variables.last_mut().unwrap().insert(name, variable);
    }
    pub fn get(&self, name: &str) -> Option<&LocalVariable> {
        for layer in self.variables.iter().rev() {
            if let Some(variable) = layer.get(name) {
                return Some(variable);
            }
        }
        None
    }
    pub fn contains_key(&self, name: &Rc<str>) -> bool {
        self.variables.last().unwrap().contains_key(name)
    }
    pub fn add_layer(&mut self) {
        self.variables.push(LocalVariableMap::new())
    }
    pub fn remove_layer(&mut self) {
        if self.variables.len() > 1 {
            self.variables.pop();
        }
    }
    pub fn push_layer(&mut self, layer: LocalVariableMap) {
        self.variables.push(layer)
    }
}

impl Default for LocalVariables {
    fn default() -> Self {
        Self::new()
    }
}

impl From<LocalVariableMap> for LocalVariables {
    fn from(value: LocalVariableMap) -> Self {
        Self {
            variables: vec![value],
        }
    }
}

impl From<Params> for LocalVariables {
    fn from(value: Params) -> Self {
        Self {
            variables: vec![value.into()],
        }
    }
}

#[derive(Clone, Debug)]
pub enum LocalVariable {
    Function(Params, Type),
    Variable(Variable),
    Other(Type),
}

impl From<Type> for LocalVariable {
    fn from(value: Type) -> Self {
        Self::Other(value)
    }
}

impl From<&Instruction> for LocalVariable {
    fn from(value: &Instruction) -> Self {
        let var_type = value.get_return_type();
        match (value, var_type) {
            (Instruction::Function(Function { params, .. }), Type::Function(function_type)) => {
                Self::Function(params.clone(), function_type.get_return_type())
            }
            (Instruction::Variable(variable), _) => Self::Variable(variable.clone()),
            (_, var_type) => Self::Other(var_type),
        }
    }
}

impl GetType for LocalVariable {
    fn get_type(&self) -> Type {
        match self {
            LocalVariable::Function(params, return_type) => FunctionType {
                return_type: return_type.clone(),
                params: params
                    .standard
                    .iter()
                    .map(|Param { var_type, name: _ }| var_type.clone())
                    .collect(),
                catch_rest: params.catch_rest.is_some(),
            }
            .into(),
            LocalVariable::Variable(variable) => variable.get_type(),
            LocalVariable::Other(var_type) => var_type.clone(),
        }
    }
}
