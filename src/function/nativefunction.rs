use std::{collections::HashMap, rc::Rc};

use super::{Function, Param, Params};
use crate::{
    error::Error,
    interpreter::Interpreter,
    variable::{Generics, GetReturnType, Type, Variable},
};
pub struct NativeFunction {
    pub params: Params,
    pub return_type: Type,
    pub body: fn(&str, &mut Interpreter) -> Result<Variable, Error>,
    pub generics: Option<Generics>,
}

impl Function for NativeFunction {
    fn exec_intern(&self, name: &str, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        (self.body)(name, interpreter)
    }
    fn get_params(&self) -> &Params {
        &self.params
    }
    fn get_generics(&self) -> Option<&Generics> {
        self.generics.as_ref()
    }
    fn simplify_generics(&self, generics: &Generics) -> Result<Rc<dyn Function>, Error> {
        let Some(current_generics) = &self.generics else {
            panic!()
        };
        let new_generics: HashMap<_, _> = current_generics
            .0
            .iter()
            .map(|(ident, types)| {
                if let Some(new_types) = generics.0.get(ident) {
                    (ident, new_types)
                } else {
                    (ident, types)
                }
            })
            .filter(|(_ident, types)| {
                types.types.len() > 1 || matches!(types.types.iter().next(), Some(&Type::Any))
            })
            .map(|(ident, types)| (ident.clone(), types.clone()))
            .collect();
        let new_generics = if !new_generics.is_empty() {
            Some(Generics(new_generics))
        } else {
            None
        };
        let new_params = self
            .params
            .standard
            .iter()
            .map(|Param { name, var_type }| Param {
                name: name.clone(),
                var_type: var_type.simplify_generics(generics),
            })
            .collect();
        Ok(Rc::new(Self {
            params: Params {
                standard: new_params,
                catch_rest: self.params.catch_rest.clone(),
            },
            return_type: self.return_type.simplify_generics(generics),
            body: self.body,
            generics: new_generics,
        }))
    }
}

impl GetReturnType for NativeFunction {
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}
