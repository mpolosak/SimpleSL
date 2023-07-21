use super::{function::Function, Instruction};
use crate::{
    function::{Param, Params},
    variable::{function_type::FunctionType, GetReturnType, GetType, Type, Variable},
};
use std::{collections::HashMap, rc::Rc};

pub type LocalVariableMap = HashMap<Rc<str>, LocalVariable>;

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
