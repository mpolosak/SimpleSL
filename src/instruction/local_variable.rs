use super::{function::Function, Instruction};
use crate::{
    function::{Param, Params},
    variable::{function_type::FunctionType, GetReturnType, GetType, Type, Variable},
};
use std::{collections::HashMap, rc::Rc};

pub type LocalVariableMap = HashMap<Rc<str>, LocalVariable>;
pub struct LocalVariables<'a> {
    variables: LocalVariableMap,
    lower_layer: Option<&'a Self>,
}

impl<'a> LocalVariables<'a> {
    pub fn new() -> Self {
        Self {
            variables: LocalVariableMap::new(),
            lower_layer: None,
        }
    }
    pub fn insert(&mut self, name: Rc<str>, variable: LocalVariable) {
        self.variables.insert(name, variable);
    }
    pub fn get(&self, name: &str) -> Option<&LocalVariable> {
        if let Some(variable) = self.variables.get(name) {
            Some(variable)
        } else if let Some(layer) = self.lower_layer {
            layer.get(name)
        } else {
            None
        }
    }
    pub fn contains_key(&self, name: &Rc<str>) -> bool {
        if self.variables.contains_key(name) {
            true
        } else if let Some(layer) = self.lower_layer {
            layer.contains_key(name)
        } else {
            false
        }
    }
    pub fn create_layer(&'a self) -> Self {
        Self {
            variables: LocalVariableMap::new(),
            lower_layer: Some(self),
        }
    }
    pub fn layer_from_map(&'a self, layer: LocalVariableMap) -> Self {
        Self {
            variables: layer,
            lower_layer: Some(self),
        }
    }
}

impl Default for LocalVariables<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl From<LocalVariableMap> for LocalVariables<'_> {
    fn from(value: LocalVariableMap) -> Self {
        Self {
            variables: value,
            lower_layer: None,
        }
    }
}

impl From<Params> for LocalVariables<'_> {
    fn from(value: Params) -> Self {
        Self {
            variables: value.into(),
            lower_layer: None,
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
                    .iter()
                    .map(|Param { var_type, name: _ }| var_type.clone())
                    .collect(),
            }
            .into(),
            LocalVariable::Variable(variable) => variable.get_type(),
            LocalVariable::Other(var_type) => var_type.clone(),
        }
    }
}
