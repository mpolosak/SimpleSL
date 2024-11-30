use super::{function::Function, Instruction};
use crate::{
    self as simplesl,
    function::{Param, Params},
    variable::{ReturnType, Type, Typed, Variable},
    Interpreter,
};
use simplesl_macros::var_type;
use std::{collections::HashMap, sync::Arc};

pub type LocalVariableMap = HashMap<Arc<str>, LocalVariable>;

#[derive(Clone)]
pub struct LocalVariables<'a> {
    variables: Vec<LocalVariableMap>,
    pub function: Vec<FunctionInfo>,
    pub in_loop: bool,
    pub interpreter: &'a Interpreter,
}

impl<'a> LocalVariables<'a> {
    #[must_use]
    pub fn new(interpreter: &'a Interpreter) -> Self {
        Self {
            variables: vec![LocalVariableMap::new()],
            function: vec![],
            interpreter,
            in_loop: false,
        }
    }

    #[must_use]
    pub fn from_params(params: Params, interpreter: &'a Interpreter) -> Self {
        Self {
            variables: vec![params.into()],
            function: vec![],
            interpreter,
            in_loop: false,
        }
    }

    pub fn insert(&mut self, name: Arc<str>, variable: LocalVariable) {
        self.variables.last_mut().unwrap().insert(name, variable);
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&LocalVariable> {
        for map in self.variables.iter().rev() {
            let var = map.get(name);
            if var.is_some() {
                return var;
            }
        }
        None
    }

    #[must_use]
    pub fn contains_key(&self, name: &Arc<str>) -> bool {
        for map in self.variables.iter().rev() {
            if map.contains_key(name) {
                return true;
            }
        }
        false
    }

    pub fn new_layer(&mut self) {
        self.push_layer(LocalVariableMap::new());
    }

    pub fn push_layer(&mut self, layer: LocalVariableMap) {
        self.variables.push(layer);
    }

    pub fn drop_layer(&mut self) {
        self.variables.pop();
    }

    pub fn function(&'a self) -> Option<&'a FunctionInfo> {
        self.function.last()
    }

    pub fn enter_function(&mut self, function: FunctionInfo) {
        self.function.push(function);
    }

    pub fn exit_function(&mut self) {
        if self.function.len() == 0 {
            panic!("Tried to exit function but not in function")
        }
        self.function.pop();
    }
}

impl<V> Extend<(Arc<str>, V)> for LocalVariables<'_>
where
    V: Into<LocalVariable>,
{
    fn extend<T: IntoIterator<Item = (Arc<str>, V)>>(&mut self, iter: T) {
        self.variables
            .last_mut()
            .unwrap()
            .extend(iter.into_iter().map(|(ident, var)| (ident, var.into())));
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
        match value {
            Instruction::Function(function) => function.into(),
            Instruction::LocalVariable(_, var) => var.clone(),
            Instruction::Variable(var) => var.clone().into(),
            ins => ins.return_type().into(),
        }
    }
}

impl From<Variable> for LocalVariable {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}

impl From<&Function> for LocalVariable {
    fn from(value: &Function) -> Self {
        Self::Function(value.params.clone(), value.return_type())
    }
}

impl Typed for LocalVariable {
    fn as_type(&self) -> Type {
        match self {
            LocalVariable::Function(params, return_type) => {
                let params: Arc<[Type]> = params
                    .iter()
                    .map(|Param { var_type, name: _ }| var_type.clone())
                    .collect();
                let return_type = return_type.clone();
                var_type!(params -> return_type)
            }
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
