use super::{Function, Params};
use crate::{
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Result,
};
pub struct NativeFunction {
    pub params: Params,
    pub return_type: Type,
    pub body: fn(&mut Interpreter) -> Result<Variable>,
}

impl Function for NativeFunction {
    fn exec_intern(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        (self.body)(interpreter)
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
