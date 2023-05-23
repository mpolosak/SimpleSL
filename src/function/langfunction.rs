use super::{Function, Line, Param};
use crate::intepreter::{Intepreter, VariableMap};
use crate::variable_type::Type;
use crate::{error::Error, variable::Variable};

#[derive(Clone)]
pub struct LangFunction {
    pub params: Vec<Param>,
    pub body: Vec<Line>,
}

impl Function for LangFunction {
    fn exec_intern(
        &self,
        _name: &str,
        intepreter: &mut Intepreter,
        mut args: VariableMap,
    ) -> Result<Variable, Error> {
        let mut to_return = Variable::Null;
        for line in &self.body {
            to_return = line.exec(intepreter, &mut args)?;
        }
        Ok(to_return)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
    fn get_return_type(&self) -> Type {
        match self.body.last() {
            Some(Line {
                result_var: _,
                instruction,
            }) => instruction.get_type(),
            None => Type::Null,
        }
    }
}
