use super::{Function, Params};
use crate::{
    error::Error,
    instruction::{Exec, Instruction},
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
};
pub struct LangFunction {
    pub params: Params,
    pub body: Box<[Instruction]>,
}

impl Function for LangFunction {
    fn exec_intern(&self, _name: &str, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        let mut result = Variable::Void;
        for instruction in self.body.iter() {
            result = instruction.exec(interpreter)?;
        }
        Ok(result)
    }
    fn get_params(&self) -> &Params {
        &self.params
    }
}

impl GetReturnType for LangFunction {
    fn get_return_type(&self) -> Type {
        match self.body.last() {
            Some(instruction) => instruction.get_return_type(),
            None => Type::Void,
        }
    }
}
