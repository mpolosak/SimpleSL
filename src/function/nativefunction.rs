use super::{Function, Params};
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
}

impl GetReturnType for NativeFunction {
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}
