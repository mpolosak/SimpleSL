use crate::parse::*;
use crate::stdfunctions::*;
use crate::iofunctions::*;
use crate::variable::*;
use crate::*;

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
    pub fn exec(&mut self, mut line: String) -> Result<Variable, String>{
        let result_var = match get_result_var(&mut line) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        let vecline = match parse(line, &self.variables) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };

        if vecline.len()<1 { return Ok(Variable::Null) }
        let result = match &vecline[0]{
            Variable::Function(function) => {
                let params = if let Some(fparams) = vecline.get(1..) { fparams.to_vec() }
                             else { Array::new() };
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