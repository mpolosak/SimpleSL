use super::{Function, Params};
use crate::{
    error::Error,
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
};
pub struct NativeFunction {
    pub params: Params,
    pub return_type: Type,
    pub body: fn(&str, &mut Interpreter) -> Result<Variable, Error>,
}

impl Function for NativeFunction {
    fn exec_intern(&self, name: &str, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        (self.body)(name, interpreter)
    }
    fn get_params(&self) -> &Params {
        &self.params
    }
}

impl GetReturnType for NativeFunction {
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}
