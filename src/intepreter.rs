use std::collections::HashMap;
use crate::params::*;
use crate::stdfunctions::*;
use crate::iofunctions::*;

type Function = fn(&mut VariableMap, ParamVec) -> Result<Variable, String>;

#[derive(Clone)]
pub enum Variable{
    Float(f64),
    Text(String),
    Function(Function),
    Null
}

pub type VariableMap = HashMap<String, Variable>;

pub struct Intepreter{
    variables:  VariableMap
}

impl Intepreter{
    pub fn new() -> Intepreter{ 
        let mut variables = VariableMap::new();
        add_io_functions(&mut variables);
        add_std_functions(&mut variables);
        Intepreter{variables}
    }
    pub fn exec(&mut self, line: String) -> Result<Variable, String>{
        let vecline = ParamVec::parse(line);
        if vecline.len()<1 { return Ok(Variable::Null) }
        let function = match &vecline[0]{
            Param::Variable(name) => match self.variables.get(name) {
                    Some(Variable::Function(func)) => *func,
                    Some(_) => return Err(String::from("First element in line should be function")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                }
            _ => return Err(String::from("First element in line should be function"))
        };
        let params = if let Some(fparams) = vecline.get(1..) { fparams.to_vec() }
                     else { ParamVec::new() };
        return function(&mut self.variables, params);
    }
}