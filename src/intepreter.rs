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

        let result = match parse(line, &mut self.variables) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };
        
        if let Some(var) = result_var {
            self.variables.insert(var, result);
            Ok(Variable::Null)
        } else {
            Ok(result)
        }
    }
}