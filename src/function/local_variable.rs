use super::{Instruction, Params};
use crate::variable_type::{GetType, Type};
use std::collections::HashMap;

pub type LocalVariableMap = HashMap<String, LocalVariable>;

#[derive(Clone)]
pub enum LocalVariable {
    Function(Params, Type),
    Other(Type),
}

impl From<Type> for LocalVariable {
    fn from(value: Type) -> Self {
        Self::Other(value)
    }
}

impl From<Instruction> for LocalVariable {
    fn from(value: Instruction) -> Self {
        let var_type = value.get_type();
        match (value, var_type) {
            (Instruction::Function { params, .. }, Type::Function { return_type, .. }) => {
                Self::Function(params, *return_type)
            }
            (_, var_type) => Self::Other(var_type),
        }
    }
}
