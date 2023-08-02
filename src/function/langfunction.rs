use super::{Function, Params};
use crate::{
    error::Error,
    instruction::Instruction,
    interpreter::Interpreter,
    variable::{Generics, GetReturnType, Type, Variable},
};
pub struct LangFunction {
    pub params: Params,
    pub body: Box<[Instruction]>,
    pub generics: Option<Generics>,
}

impl Function for LangFunction {
    fn exec_intern(&self, _name: &str, interpreter: &mut Interpreter) -> Result<Variable, Error> {
        interpreter.exec(&self.body)
    }
    fn get_params(&self) -> &Params {
        &self.params
    }
    fn get_generics(&self) -> Option<&Generics> {
        self.generics.as_ref()
    }
    fn simplify_generics(&self, _generics: &Generics) -> Result<std::rc::Rc<dyn Function>, Error> {
        todo!()
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
