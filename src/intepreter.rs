use std::collections::HashMap;
use std::io;
use crate::params::*;

type Function = fn(&mut VariableMap, ParamVec);

enum Variable{
    Float(f64),
    Text(String),
    Function(Function)
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
                        _ => return eprintln!("Variable {} doesn't exist", name)
                    }
                };
            }
            println!("{}", text);
        }));
        variables.insert(String::from("cgetline"), Variable::Function(|variables, params|{
            if params.len()!=1 {
                return eprintln!("Function cgetline requiers exactly 1 argument");
            }
            if let Param::Variable(var) = &params[0]{
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Unable to read user input");
                input = input.replace("\n", "");
                variables.insert(var.to_string(), Variable::Text(input));
            } else {
                eprintln!("Function cgetline requires variable name as argument");
            }
        }));
        variables.insert(String::from("add"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return println!("Function add requiers 2 arguments")
            }
            let var1name = if let Param::Variable(name) = &params[0] {
                name.to_string()
            } else {
                return eprintln!("First parameter should be variable name");
            };
            let var2name = if let Param::Variable(name) = &params[1] {
                name.to_string()
            } else {
                return eprintln!("Second parameter should be variable name");
            };
            let mut var1 = match variables.get(&var1name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return println!("Function add requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            };
            let var2 = match variables.get(&var2name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return println!("Function add requiers float"),
                _ => return println!("Variable {} doesn't exist", var2name),
            };
            var1+=var2;
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("multiply"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return println!("Function multiply requiers 2 arguments")
            }
            let var1name = if let Param::Variable(name) = &params[0] {
                name.to_string()
            } else {
                return eprintln!("First parameter should be variable name");
            };
            let var2name = if let Param::Variable(name) = &params[1] {
                name.to_string()
            } else {
                return eprintln!("Second parameter should be variable name");
            };
            let mut var1 = match variables.get(&var1name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return println!("Function multiply requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            };
            let var2 = match variables.get(&var2name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return println!("Function multiply requiers float"),
                _ => return println!("Variable {} doesn't exist", var2name),
            };
            var1*=var2;
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("divide"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return println!("Function divide requiers 2 arguments")
            }
            let var1name = if let Param::Variable(name) = &params[0] {
                name.to_string()
            } else {
                return eprintln!("First parameter should be variable name");
            };
            let var2name = if let Param::Variable(name) = &params[1] {
                name.to_string()
            } else {
                return eprintln!("Second parameter should be variable name");
            };
            let mut var1 = match variables.get(&var1name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return println!("Function divide requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            };
            let var2 = match variables.get(&var2name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return println!("Function divide requiers float"),
                _ => return println!("Variable {} doesn't exist", var2name),
            };
            var1/=var2;
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("or"), Variable::Function(|variables, params|{
            if params.len()!=2{
                return eprintln!("Function or requiers 2 arguments")
            }
            let var1name = if let Param::Variable(name) = &params[0] {
                name.to_string()
            } else {
                return eprintln!("First parameter should be variable name");
            };
            let var2name = if let Param::Variable(name) = &params[1] {
                name.to_string()
            } else {
                return eprintln!("Second parameter should be variable name");
            };
            let mut var1 = match variables.get(&var1name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return eprintln!("Function or requiers float"),
                _ => return eprintln!("Variable {} doesn't exist", var1name),
            };
            let var2 = match variables.get(&var2name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return eprintln!("Function or requiers float"),
                _ => return eprintln!("Variable {} doesn't exist", var2name),
            };
            var1=var1.abs()+var2.abs();
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("not"), Variable::Function(|variables, params|{
            if params.len()!=1{
                return eprintln!("Function not requiers 1 argument")
            }
            let var_name = if let Param::Variable(name) = &params[0] {
                name.to_string()
            } else {
                return eprintln!("Parameter should be variable name");
            };
            let mut var = match variables.get(&var_name) {
                Some(Variable::Float(value)) => *value,
                Some(_) => return eprintln!("Function not requiers float"),
                _ => return eprintln!("Variable {} doesn't exist", var_name),
            };
            var = if var == 0.0 { 1.0 } else { 0.0 };
            variables.insert(var_name, Variable::Float(var));
        }));
        variables.insert(String::from("if"), Variable::Function(|variables, params|{
            if params.len()<2{
                return eprintln!("Function if requiers at least 2 arguments")
            }
            let condition = match &params[0]{
                Param::Float(value) => *value,
                Param::Variable(name) => match variables.get(name) {
                        Some(Variable::Float(value)) => *value,
                        Some(_) => return eprintln!("First argument to function if should be float"),
                        _ => return eprintln!("Variable {} doesn't exist", name),
                    }
                _ => return eprintln!("First argument to function if should be float")
            };
            if condition == 0.0 { return };
            let function = match &params[1]{
                Param::Variable(name) => match variables.get(name) {
                        Some(Variable::Function(func)) => *func,
                        Some(_) => return eprintln!("Second argument to function if should be function"),
                        _ => return eprintln!("Variable {} doesn't exist", name),
                    }
                _ => return eprintln!("Second argument to function if should be function")
            };
            let function_params = if let Some(fparams) = params.get(2..) { fparams.to_vec() }
                                  else { ParamVec::new() };
            function(variables, function_params);
        }));
        variables.insert(String::from("while"), Variable::Function(|variables, params|{
            loop{
                let condition = match &params[0]{
                    Param::Float(value) => *value,
                    Param::Variable(name) => match variables.get(name) {
                            Some(Variable::Float(value)) => *value,
                            Some(_) => return eprintln!("First argument to function while should be float"),
                            _ => return eprintln!("Variable {} doesn't exist", name),
                        }
                    _ => return eprintln!("First argument to function while should be float")
                };
                if condition == 0.0 { break };
                let function = match &params[1]{
                    Param::Variable(name) => match variables.get(name) {
                            Some(Variable::Function(func)) => *func,
                            Some(_) => return eprintln!("Second argument to function if should be function"),
                            _ => return eprintln!("Variable {} doesn't exist", name),
                        }
                    _ => return eprintln!("Second argument to function if should be function")
                };
                let function_params = if let Some(fparams) = params.get(2..) { fparams.to_vec() }
                                      else { ParamVec::new() };
                function(variables, function_params);
            }
        }));
        Intepreter{variables}
    }
    pub fn exec(&mut self, line: String){
        let vecline = ParamVec::parse(line);
        if vecline.len()<1 { return }
        let function = match &vecline[0]{
            Param::Variable(name) => match self.variables.get(name) {
                    Some(Variable::Function(func)) => *func,
                    Some(_) => return eprintln!("First element in line should be function"),
                    _ => return eprintln!("Variable {} doesn't exist", name),
                }
            _ => return eprintln!("First element in line should be function")
        };
        let params = if let Some(fparams) = vecline.get(1..) { fparams.to_vec() }
                     else { ParamVec::new() };
        function(&mut self.variables, params);
    }
}