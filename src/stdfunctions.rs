use crate::intepreter::*;
use crate::params::*;

pub fn add_std_functions(variables: &mut VariableMap){
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
}