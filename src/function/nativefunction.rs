use crate::function::{Function,Param};
use crate::intepreter::{Intepreter,VariableMap};
use crate::variable::Variable;
use crate::error::Error;

#[derive(Clone)]
pub struct NativeFunction {
    pub params: Vec<Param>,
    pub body: fn(String, &mut Intepreter, VariableMap) -> Result<Variable, Error>,
}

impl Function for NativeFunction {
    fn exec_intern(&self, name: String, intepreter: &mut Intepreter,
        args: VariableMap) -> Result<Variable, Error>{
        (self.body)(name, intepreter, args)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
}