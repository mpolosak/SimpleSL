use super::{Function, Params};
use crate::{
    instruction::Instruction,
    interpreter::Interpreter,
    variable::{GetReturnType, Type, Variable},
    Result,
};
pub struct LangFunction {
    pub params: Params,
    pub body: Box<[Instruction]>,
}

impl Function for LangFunction {
    fn exec_intern(&self, interpreter: &mut Interpreter) -> Result<Variable> {
        interpreter.exec(&self.body)
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
