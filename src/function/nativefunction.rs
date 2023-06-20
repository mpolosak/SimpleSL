use super::{Function, Params};
use crate::interpreter::{Interpreter, VariableMap};
use crate::variable_type::Type;
use crate::{error::Error, variable::Variable};

#[derive(Clone)]
pub struct NativeFunction {
    pub params: Params,
    pub return_type: Type,
    pub body: fn(&str, &mut Interpreter, VariableMap) -> Result<Variable, Error>,
}

impl Function for NativeFunction {
    fn exec_intern(
        &self,
        name: &str,
        intepreter: &mut Interpreter,
        args: VariableMap,
    ) -> Result<Variable, Error> {
        (self.body)(name, intepreter, args)
    }
    fn get_params(&self) -> &Params {
        &self.params
    }
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}
