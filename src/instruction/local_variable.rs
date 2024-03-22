use super::{function::AnonymousFunction, Instruction};
use crate::{
    function::{Param, Params},
    variable::{FunctionType, ReturnType, Type, Typed, Variable},
};
use std::{collections::HashMap, sync::Arc};

pub type LocalVariableMap = HashMap<Arc<str>, LocalVariable>;
pub struct LocalVariables<'a> {
    variables: LocalVariableMap,
    lower_layer: Option<&'a Self>,
    function: Option<FunctionInfo>,
}

impl<'a> LocalVariables<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            variables: LocalVariableMap::new(),
            lower_layer: None,
            function: None,
        }
    }
    pub fn insert(&mut self, name: Arc<str>, variable: LocalVariable) {
        self.variables.insert(name, variable);
    }
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&LocalVariable> {
        self.variables
            .get(name)
            .or_else(|| self.lower_layer?.get(name))
    }
    #[must_use]
    pub fn contains_key(&self, name: &Arc<str>) -> bool {
        self.variables.contains_key(name)
            || self
                .lower_layer
                .is_some_and(|layer| layer.contains_key(name))
    }
    #[must_use]
    pub fn create_layer(&'a self) -> Self {
        Self {
            variables: LocalVariableMap::new(),
            lower_layer: Some(self),
            function: None,
        }
    }

    #[must_use]
    pub fn function_layer(&'a self, layer: LocalVariableMap, function: FunctionInfo) -> Self {
        Self {
            variables: layer,
            lower_layer: Some(self),
            function: Some(function),
        }
    }

    pub fn function(&'a self) -> Option<&FunctionInfo> {
        self.function
            .as_ref()
            .or_else(|| self.lower_layer.and_then(LocalVariables::function))
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
            function: None,
        }
    }
}

impl From<Params> for LocalVariables<'_> {
    fn from(value: Params) -> Self {
        Self {
            variables: value.into(),
            lower_layer: None,
            function: None,
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
        let Instruction::Variable(variable) = value else {
            return Self::Other(value.return_type());
        };
        Self::Variable(variable.clone())
    }
}

impl From<Variable> for LocalVariable {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

impl From<&AnonymousFunction> for LocalVariable {
    fn from(value: &AnonymousFunction) -> Self {
        Self::Function(value.params.clone(), value.return_type())
    }
}

impl Typed for LocalVariable {
    fn as_type(&self) -> Type {
        match self {
            LocalVariable::Function(params, return_type) => FunctionType {
                return_type: return_type.clone(),
                params: params
                    .iter()
                    .map(|Param { var_type, name: _ }| var_type.clone())
                    .collect(),
            }
            .into(),
            LocalVariable::Variable(variable) => variable.as_type(),
            LocalVariable::Other(var_type) => var_type.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionInfo {
    name: Option<Arc<str>>,
    return_type: Type,
}

impl FunctionInfo {
    pub fn new(name: Option<Arc<str>>, return_type: Type) -> Self {
        Self { name, return_type }
    }

    pub fn name(&self) -> Option<Arc<str>> {
        self.name.clone()
    }

    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}
