use std::collections::HashMap;

type Function = fn(&mut VariableMap, String);

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
        variables.insert(String::from("print"), Variable::Function(|variables, param|{
            match variables.get(&param) {
                Some(variable) => print_variable(variable),
                _ => println!("Variable doesn't exist"),
            }
        }));
        variables.insert(String::from("setf"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.splitn(2, ' ').collect();
            let varname = String::from(vecparam[0]);
            let svalue = if vecparam.len() == 2 {
                String::from(vecparam[1])
            }   else {
                String::new()
            };
            match svalue.parse::<f64>(){
                Ok(value) => {variables.insert(varname, Variable::Float(value));},
                _ => println!("'{}' isn't parsable to float", svalue)
            }
        }));
        variables.insert(String::from("sets"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.splitn(2, ' ').collect();
            let varname = String::from(vecparam[0]);
            let value = if vecparam.len() == 2 {
                String::from(vecparam[1])
            }   else {
                String::new()
            };
            variables.insert(varname, Variable::Text(value));
        }));
        variables.insert(String::from("add"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.split(' ').collect();
            if vecparam.len()!=2{
                return println!("Function add requiers 2 arguments")
            }
            let var1name = String::from(vecparam[0]);
            let mut var1: f64;
            match variables.get(&var1name) {
                Some(Variable::Float(value)) => var1=*value,
                Some(_) => return println!("Function add requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            }
            let var2name = String::from(vecparam[1]);
            let mut var2: f64;
            match variables.get(&var2name) {
                Some(Variable::Float(value)) => var2=*value,
                Some(_) => return println!("Function add requiers float"),
                _ => return println!("Variable {} doesn't exist", var2name),
            }
            var1+=var2;
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("multiply"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.split(' ').collect();
            if vecparam.len()!=2{
                return println!("Function multiply requiers 2 arguments")
            }
            let var1name = String::from(vecparam[0]);
            let mut var1: f64;
            match variables.get(&var1name) {
                Some(Variable::Float(value)) => var1=*value,
                Some(_) => return println!("Function multiply requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            }
            let var2name = String::from(vecparam[1]);
            let mut var2: f64;
            match variables.get(&var2name) {
                Some(Variable::Float(value)) => var2=*value,
                Some(_) => return println!("Function multiply requiers float"),
                _ => return println!("Variable {} doesn't exist", var2name),
            }
            var1*=var2;
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("divide"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.split(' ').collect();
            if vecparam.len()!=2{
                return println!("Function add requiers 2 arguments")
            }
            let var1name = String::from(vecparam[0]);
            let mut var1: f64;
            match variables.get(&var1name) {
                Some(Variable::Float(value)) => var1=*value,
                Some(_) => return println!("Function add requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            }
            let var2name = String::from(vecparam[1]);
            let mut var2: f64;
            match variables.get(&var2name) {
                Some(Variable::Float(value)) => var2=*value,
                Some(_) => return println!("Function divide requiers float"),
                _ => return println!("Variable {} doesn't exist", var2name),
            }
            var1/=var2;
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("or"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.split(' ').collect();
            if vecparam.len()!=2{
                return println!("Function or requiers 2 arguments")
            }
            let var1name = String::from(vecparam[0]);
            let mut var1: f64;
            match variables.get(&var1name) {
                Some(Variable::Float(value)) => var1=*value,
                Some(_) => return println!("Function or requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            }
            let var2name = String::from(vecparam[1]);
            let mut var2: f64;
            match variables.get(&var2name) {
                Some(Variable::Float(value)) => var2=*value,
                Some(_) => return println!("Function or requiers float"),
                _ => return println!("Variable {} doesn't exist", var2name),
            }
            var1=var1.abs()+var2.abs();
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("not"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.split(' ').collect();
            if vecparam.len()!=1{
                return println!("Function not requiers 1 argument")
            }
            let var1name = String::from(vecparam[0]);
            let mut var1: f64;
            match variables.get(&var1name) {
                Some(Variable::Float(value)) => var1=*value,
                Some(_) => return println!("Function not requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            }
            var1=if var1==0.0 { 1.0 } else { 0.0 };
            variables.insert(var1name, Variable::Float(var1));
        }));
        variables.insert(String::from("if"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.splitn(2, ' ').collect();
            let var1name = String::from(vecparam[0]);
            let mut var1: f64;
            match variables.get(&var1name) {
                Some(Variable::Float(value)) => var1=*value,
                Some(_) => return println!("Function if requiers float"),
                _ => return println!("Variable {} doesn't exist", var1name),
            }
            if var1 != 0.0 {
                let var2 = String::from(vecparam[1]);
                let vecvar2: Vec<&str> = var2.splitn(2, ' ').collect();
                let execvarname = String::from(vecvar2[0]);
                let param = if vecvar2.len() == 2 {
                    String::from(vecvar2[1])
                }   else {
                    String::new()
                };
                match variables.get(&execvarname) {
                    Some(Variable::Function(function)) => function(variables, param),
                    Some(_) => println!("Not the function"),
                    _ => println!("Variable doesn't exist"),
                }
            }
        }));
        variables.insert(String::from("while"), Variable::Function(|variables, param|{
            let vecparam: Vec<&str> = param.splitn(2, ' ').collect();
            let var1name = String::from(vecparam[0]);
            loop{
                let mut var1: f64;
                match variables.get(&var1name) {
                    Some(Variable::Float(value)) => var1=*value,
                    Some(_) => return println!("Function if requiers float"),
                    _ => return println!("Variable {} doesn't exist", var1name),
                }
                if var1 == 0.0 { break };
                let var2 = String::from(vecparam[1]);
                let vecvar2: Vec<&str> = var2.splitn(2, ' ').collect();
                let execvarname = String::from(vecvar2[0]);
                let param = if vecvar2.len() == 2 {
                    String::from(vecvar2[1])
                }   else {
                    String::new()
                };
                match variables.get(&execvarname) {
                    Some(Variable::Function(function)) => function(variables, param),
                    Some(_) => println!("Not the function"),
                    _ => println!("Variable doesn't exist"),
                }
            }
        }));
        Intepreter{variables}
    }
    pub fn exec(&mut self, line: String){
        let vecline: Vec<&str> = line.splitn(2, ' ').collect();
        let varname = String::from(vecline[0]);
        let param = if vecline.len() == 2 {
            String::from(vecline[1])
        }   else {
            String::new()
        };
        match self.variables.get(&varname) {
            Some(Variable::Function(function)) => function(&mut self.variables, param),
            Some(_) => println!("Not the function"),
            _ => println!("Variable doesn't exist"),
        }
    }
}

fn print_variable(variable: &Variable){
    match variable{
        Variable::Float(number) => println!("{}", number),
        Variable::Text(text) => println!("{}", text),
        Variable::Function(_) => println!("Function")
    }
}