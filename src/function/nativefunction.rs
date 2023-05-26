use super::{Function, Param};
use crate::intepreter::{Intepreter, VariableMap};
use crate::variable_type::Type;
use crate::{error::Error, variable::Variable};

#[derive(Clone)]
pub struct NativeFunction {
    pub params: Vec<Param>,
    pub return_type: Type,
    pub body: fn(&str, &mut Intepreter, VariableMap) -> Result<Variable, Error>,
}

impl Function for NativeFunction {
    fn exec_intern(
        &self,
        name: &str,
        intepreter: &mut Intepreter,
        args: VariableMap,
    ) -> Result<Variable, Error> {
        (self.body)(name, intepreter, args)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
    fn get_return_type(&self) -> Type {
        self.return_type.clone()
    }
}
