use super::{Function, Param};
use super::instruction::Line;
use crate::intepreter::{Intepreter, VariableMap};
use crate::{variable::Variable,error::Error};

#[derive(Clone)]
pub struct LangFunction {
    pub params: Vec<Param>,
    pub body: Vec<Line>
}

impl Function for LangFunction {
    fn exec_intern(&self, _name: String, intepreter: &mut Intepreter,
            mut args: VariableMap) -> Result<Variable, Error> {
        let mut to_return = Variable::Null;
        for Line{result_var, instruction} in &self.body{
            let result = instruction.exec(intepreter, &args)?;
            if let Some(var) = result_var{
                args.insert(var, result);
            } else {
                to_return = result;
            }
        }
        Ok(to_return)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
}