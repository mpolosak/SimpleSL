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