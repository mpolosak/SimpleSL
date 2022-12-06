use std::collections::HashMap;
use crate::params::*;
use crate::stdfunctions::*;
use crate::iofunctions::*;
use crate::*;

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
        let (result_var, vecline) = match ParamVec::parse(line) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };
        if vecline.len()<1 { return Ok(Variable::Null) }
        let result = match get_var!(self.variables, vecline[0]){
            Variable::Function(function) => {
                let params = if let Some(fparams) = vecline.get(1..) { fparams.to_vec() }
                             else { ParamVec::new() };
                function(&mut self.variables, params)
            },
            value => Ok(value.clone()),
        };
        
        match result_var {
            Some(var) => {
                match result {
                    Ok(value) => {
                        self.variables.insert(var, value);
                        Ok(Variable::Null)
                    },
                    Err(e) => Err(e),
                }
            }
            None => result,
        }
    }
}