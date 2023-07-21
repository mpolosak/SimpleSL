use super::{function::Function, Instruction};
use crate::{
    function::{Param, Params},
    variable::{GetReturnType, GetType, Type, Variable},
};
use std::{collections::HashMap, rc::Rc};

pub type LocalVariableMap = HashMap<Rc<str>, LocalVariable>;

#[derive(Clone)]
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

impl From<Instruction> for LocalVariable {
    fn from(value: Instruction) -> Self {
        let var_type = value.get_return_type();
        match (value, var_type) {
            (
                Instruction::Function(Function { params, .. }),
                Type::Function { return_type, .. },
            ) => Self::Function(params, *return_type),
            (Instruction::Variable(variable), _) => Self::Variable(variable),
            (_, var_type) => Self::Other(var_type),
        }
    }
}

impl GetType for LocalVariable {
    fn get_type(&self) -> Type {
        match self {
            LocalVariable::Function(params, return_type) => Type::Function {
                return_type: return_type.clone().into(),
                params: params
                    .standard
                    .iter()
                    .map(|Param { var_type, name: _ }| var_type.clone())
                    .collect(),
                catch_rest: params.catch_rest.is_some(),
            },
            LocalVariable::Variable(variable) => variable.get_type(),
            LocalVariable::Other(var_type) => var_type.clone(),
        }
    }
}
