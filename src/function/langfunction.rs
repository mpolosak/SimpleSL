use super::{Function, Params};
use crate::interpreter::{Interpreter, VariableMap};
use crate::variable_type::{GetType, Type};
use crate::{error::Error, instruction::Instruction, variable::Variable};

#[derive(Clone)]
pub struct LangFunction {
    pub params: Params,
    pub body: Vec<Instruction>,
}

impl Function for LangFunction {
    fn exec_intern(
        &self,
        _name: &str,
        intepreter: &mut Interpreter,
        mut args: VariableMap,
    ) -> Result<Variable, Error> {
        let mut to_return = Variable::Null;
        for line in &self.body {
            to_return = line.exec(intepreter, &mut args)?;
        }
        Ok(to_return)
    }
    fn get_params(&self) -> &Params {
        &self.params
    }
    fn get_return_type(&self) -> Type {
        match self.body.last() {
            Some(instruction) => instruction.get_type(),
            None => Type::Null,
        }
    }
}
