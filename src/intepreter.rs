use std::collections::HashMap;
use std::io;
use crate::params::*;

type Function = fn(&mut VariableMap, ParamVec) -> Result<Variable, String>;

#[derive(Clone)]
pub enum Variable{
    Float(f64),
    Text(String),
    Function(Function),
    Null
}

type VariableMap = HashMap<String, Variable>;

pub struct Intepreter{
    variables:  VariableMap
}

impl Intepreter{
    pub fn new() -> Intepreter{
        let mut variables = VariableMap::new();
        variables.insert(String::from("print"), Variable::Function(|variables, params|{
            let mut text = String::new();
            for param in params {
                match param {
                    Param::Float(value) => text+=&value.to_string(),
                    Param::Text(value) => text+=&value,
                    Param::Variable(name) => match variables.get(&name){
                        Some(Variable::Float(value)) => text+=&value.to_string(),
                        Some(Variable::Text(value)) => text+=&value,
                        Some(Variable::Function(_)) => text+="Function",
                        _ => return Err(format!("Variable {} doesn't exist", name))
                    }
                };
            }
            println!("{}", text);
            Ok(Variable::Null)
        }));
        variables.insert(String::from("cgetline"), Variable::Function(|_variables, _params|{
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Unable to read user input");
            input = input.replace("\n", "");
            Ok(Variable::Text(input))
        }));
        variables.insert(String::from("add"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return Err(String::from("Function add requiers 2 arguments"));
            }
            let var1 = match &params[0]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function add requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function add requiers float"))
            };
            let var2 = match &params[1]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function add requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function add requiers float"))
            };
            Ok(Variable::Float(var1+var2))
        }));
        variables.insert(String::from("multiply"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return Err(String::from("Function multiply requiers 2 arguments"));
            }
            let var1 = match &params[0]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function multiply requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function add requiers float"))
            };
            let var2 = match &params[1]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function multiply requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function add requiers float"))
            };
            Ok(Variable::Float(var1*var2))
        }));
        variables.insert(String::from("divide"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return Err(String::from("Function divide requiers 2 arguments"))
            }
            let var1 = match &params[0]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function divide requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function divide requiers float"))
            };
            let var2 = match &params[1]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function divide requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function divide requiers float"))
            };
            Ok(Variable::Float(var1/var2))
        }));
        variables.insert(String::from("or"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return Err(String::from("Function or requiers 2 arguments"));
            }
            let var1 = match &params[0]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function or requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function or requiers float"))
            };
            let var2 = match &params[1]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function or requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function or requiers float"))
            };
            Ok(Variable::Float(var1.abs()+var2.abs()))
        }));
        variables.insert(String::from("not"), Variable::Function(|variables, params|{
            if params.len()!=1{
                return Err(String::from("Function not requiers 1 argument"));
            }
            let var = match &params[0]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                    Some(Variable::Float(value)) => *value,
                    Some(_) => return Err(String::from("Function not requiers float")),
                    _ => return Err(format!("Variable {} doesn't exist", name)),
                },
                _ => return Err(String::from("Function not requiers float"))
            };
            Ok(Variable::Float(if var==0.0{1.0}else{0.0}))
        }));
        variables.insert(String::from("if"), Variable::Function(|variables, params|{
            if params.len()<2{
                return Err(String::from("Function if requiers at least 2 arguments"));
            }
            let condition = match &params[0]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                        Some(Variable::Float(value)) => *value,
                        Some(_) => return Err(String::from("First argument to function if should be float")),
                        _ => return Err(format!("Variable {} doesn't exist", name)),
                    }
                _ => return Err(String::from("First argument to function if should be float"))
            };
            if condition == 0.0 { return Ok(Variable::Null)};
            let function = match &params[1]{
                Param::Variable(name) => match variables.get(name) {
                        Some(Variable::Function(func)) => *func,
                        Some(_) => return Err(String::from("Second argument to function if should be function")),
                        _ => return Err(format!("Variable {} doesn't exist", name)),
                    }
                _ => return Err(String::from("Second argument to function if should be function"))
            };
            let function_params = if let Some(fparams) = params.get(2..) { fparams.to_vec() }
                                  else { ParamVec::new() };
            return function(variables, function_params);
        }));
        variables.insert(String::from("while"), Variable::Function(|variables, params|{
            loop{
                let condition = match &params[0]{
                    Param::Float(value) => *value,
                    Param::Variable(name) => match variables.get(name) {
                            Some(Variable::Float(value)) => *value,
                            Some(_) => return Err(String::from("First argument to function while should be float")),
                            _ => return Err(format!("Variable {} doesn't exist", name)),
                        }
                    _ => return Err(String::from("First argument to function while should be float"))
                };
                if condition == 0.0 { break };
                let function = match &params[1]{
                    Param::Variable(name) => match variables.get(name) {
                            Some(Variable::Function(func)) => *func,
                            Some(_) => return Err(String::from("Second argument to function if should be function")),
                            _ => return Err(format!("Variable {} doesn't exist", name)),
                        }
                    _ => return Err(String::from("Second argument to function if should be function"))
                };
                let function_params = if let Some(fparams) = params.get(2..) { fparams.to_vec() }
                                      else { ParamVec::new() };
                if let Err(error) = function(variables, function_params){
                    return Err(error)
                }
            }
            return Ok(Variable::Null)
        }));
        variables.insert(String::from("move"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return Err(String::from("Function move requieres exactly 2 arguments"));
            }
            let var1 = if let Param::Variable(name) = &params[0] {
                name.clone()
            } else {
                return Err(String::from("First argument to function move should be variable"));
            };
            let var2 = match &params[1]{
                Param::Float(value) => Variable::Float(*value),
                Param::Text(value) => Variable::Text(String::from(value)),
                Param::Variable(name) => match variables.get(name) {
                    Some(variable) => variable.clone(),
                    _ => return Err(format!("Variable {} does not exist", name))
                }
            };
            variables.insert(var1, var2);
            Ok(Variable::Null)
        }));
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