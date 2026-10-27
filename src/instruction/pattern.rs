use crate::{
    Error, Interpreter,
    function::Param,
    instruction::{
        local_variable::{LocalVariable, LocalVariables},
        pattern::destruct_pattern::DestructPattern,
    },
    interpreter::VariableMap,
    variable::{Array, Type, Typed, Variable},
};
use pest::iterators::Pair;
use simplesl_parser::Rule;
use std::{collections::HashMap, sync::Arc};
pub mod destruct_pattern;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub destruct_pattern: DestructPattern,
    pub var_type: Type,
}

impl Pattern {
    pub fn new_ident_pattern(ident: Arc<str>, var_type: Type) -> Self {
        Pattern {
            destruct_pattern: DestructPattern::Ident(ident),
            var_type,
        }
    }

    pub fn create_instruction(pair: Pair<Rule>, ex_type: &Type) -> Result<Self, Error> {
        let str = pair.as_str().into();
        let mut inner = pair.into_inner();
        let destruct_pattern_pair = inner.next().unwrap();
        let destruct_pattern = DestructPattern::create_instruction(destruct_pattern_pair);
        let var_type = inner.next().map(Type::from);
        let destruct_pattern_type = destruct_pattern.as_type();
        let Some(var_type) = var_type else {
            let ct = destruct_pattern_type.conjoin(ex_type);
            let var_type = if ct != Type::Never {
                ct
            } else {
                destruct_pattern_type
            };
            return Ok(Self {
                destruct_pattern,
                var_type,
            });
        };
        if !var_type.matches(&destruct_pattern_type) {
            return Err(Error::SelfContradictoryPattern(str));
        }

        Ok(Self {
            destruct_pattern,
            var_type,
        })
    }

    pub fn is_matched(&self, var_type: &Type) -> bool {
        match &self.destruct_pattern {
            DestructPattern::Struct(fields) => {
                struct_pattern_matches(fields, &self.var_type, var_type)
            }
            DestructPattern::Array { .. } => array_pattern_matches(&self.var_type, var_type),
            _ => var_type.matches(&self.var_type),
        }
    }

    pub fn matches_variable(&self, variable: &Variable) -> bool {
        self.is_matched(&variable.as_type())
            && match (&self.destruct_pattern, variable) {
                (DestructPattern::Array { items, rest }, Variable::Array(array)) => {
                    if rest.is_some() {
                        array.len() >= items.len()
                    } else {
                        items.len() == array.len()
                    }
                }
                (DestructPattern::Array { .. }, _) => false,
                _ => true,
            }
    }

    pub fn insert_local_variables(&self, local_variables: &mut LocalVariables) {
        match &self.destruct_pattern {
            DestructPattern::Ident(ident) => {
                local_variables.insert(ident.clone(), LocalVariable::Other(self.var_type.clone()))
            }
            DestructPattern::Tuple(idents) => {
                let tuple = self.var_type.clone().flatten_tuple().unwrap();
                for (ident, var_type) in idents.iter().cloned().zip(tuple.iter().cloned()) {
                    local_variables.insert(ident, LocalVariable::Other(var_type));
                }
            }
            DestructPattern::Array { items, rest } => {
                let element_type = self.var_type.element_type().unwrap_or(Type::Any);
                for ident in items.iter().cloned() {
                    local_variables.insert(ident, LocalVariable::Other(element_type.clone()));
                }
                if let Some(rest) = rest {
                    local_variables.insert(
                        rest.clone(),
                        LocalVariable::Other(Type::Array(element_type.into())),
                    );
                }
            }
            DestructPattern::Struct(fields) => {
                for (field, ident) in fields.iter() {
                    local_variables.insert(
                        ident.clone(),
                        LocalVariable::Other(self.var_type.field_type(field).unwrap()),
                    );
                }
            }
        }
    }

    pub fn insert_variables(&self, interpreter: &mut Interpreter, variable: Variable) {
        match &self.destruct_pattern {
            DestructPattern::Ident(ident) => interpreter.insert(ident.clone(), variable),
            DestructPattern::Tuple(idents) => {
                let tuple = variable.into_tuple().unwrap();
                for (ident, var) in idents.iter().cloned().zip(tuple.iter().cloned()) {
                    interpreter.insert(ident, var);
                }
            }
            DestructPattern::Array { items, rest } => {
                let array = variable.into_array().unwrap();
                for (ident, var) in items.iter().cloned().zip(array.iter().cloned()) {
                    interpreter.insert(ident, var);
                }
                if let Some(rest) = rest {
                    interpreter.insert(
                        rest.clone(),
                        Array::new_with_type(
                            array.element_type().clone(),
                            array[items.len()..].to_vec().into(),
                        )
                        .into(),
                    );
                }
            }
            DestructPattern::Struct(fields) => {
                let struct_fields = variable.into_struct().unwrap();
                for (field, ident) in fields.iter() {
                    interpreter.insert(ident.clone(), struct_fields[field].clone());
                }
            }
        }
    }

    pub fn insert_variables_into_vm(&self, vm: &mut VariableMap, variable: Variable) {
        match &self.destruct_pattern {
            DestructPattern::Ident(ident) => {
                vm.insert(ident.clone(), variable);
            }
            DestructPattern::Tuple(idents) => {
                let tuple = variable.into_tuple().unwrap();
                for (ident, var) in idents.iter().cloned().zip(tuple.iter().cloned()) {
                    vm.insert(ident, var);
                }
            }
            DestructPattern::Array { items, rest } => {
                let array = variable.into_array().unwrap();
                for (ident, var) in items.iter().cloned().zip(array.iter().cloned()) {
                    vm.insert(ident, var);
                }
                if let Some(rest) = rest {
                    vm.insert(
                        rest.clone(),
                        Array::new_with_type(
                            array.element_type().clone(),
                            array[items.len()..].to_vec().into(),
                        )
                        .into(),
                    );
                }
            }
            DestructPattern::Struct(fields) => {
                let struct_fields = variable.into_struct().unwrap();
                for (field, ident) in fields.iter() {
                    vm.insert(ident.clone(), struct_fields[field].clone());
                }
            }
        }
    }

    pub fn insert_types(&self, tm: &mut HashMap<Arc<str>, Type>) {
        match &self.destruct_pattern {
            DestructPattern::Ident(ident) => {
                tm.insert(ident.clone(), self.var_type.clone());
            }
            DestructPattern::Tuple(idents) => {
                let tuple = self.var_type.clone().flatten_tuple().unwrap();
                for (ident, var) in idents.iter().cloned().zip(tuple.iter().cloned()) {
                    tm.insert(ident, var);
                }
            }
            DestructPattern::Array { items, rest } => {
                let element_type = self.var_type.element_type().unwrap_or(Type::Any);
                for ident in items.iter().cloned() {
                    tm.insert(ident, element_type.clone());
                }
                if let Some(rest) = rest {
                    tm.insert(rest.clone(), Type::Array(element_type.into()));
                }
            }
            DestructPattern::Struct(fields) => {
                for (field, ident) in fields.iter() {
                    tm.insert(ident.clone(), self.var_type.field_type(field).unwrap());
                }
            }
        }
    }
}

fn array_pattern_matches(pattern_type: &Type, value_type: &Type) -> bool {
    match value_type {
        Type::Multi(types) => types
            .iter()
            .any(|type_| array_pattern_matches(pattern_type, type_)),
        Type::Array(_) => value_type.matches(pattern_type),
        _ => false,
    }
}

fn struct_pattern_matches(
    fields: &[(Arc<str>, Arc<str>)],
    pattern_type: &Type,
    value_type: &Type,
) -> bool {
    match value_type {
        Type::Multi(types) => types
            .iter()
            .any(|type_| struct_pattern_matches(fields, pattern_type, type_)),
        Type::Struct(value_struct) => fields.iter().all(|(field, _)| {
            pattern_type
                .field_type(field)
                .is_some_and(|pattern_field_type| {
                    value_struct.0.get(field).is_some_and(|value_field_type| {
                        pattern_field_type.matches(value_field_type)
                    })
                })
        }),
        _ => false,
    }
}

impl From<Param> for Pattern {
    fn from(value: Param) -> Self {
        Self {
            destruct_pattern: DestructPattern::Ident(value.name),
            var_type: value.var_type,
        }
    }
}
