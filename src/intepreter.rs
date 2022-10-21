use std::collections::HashMap;

type Function = fn(&VariableMap, String);

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
        Intepreter{variables}
    }
    pub fn exec(&self, line: String){
        let vecline: Vec<&str> = line.splitn(2, ' ').collect();
        let varname = String::from(vecline[0]);
        let param = String::from(vecline[1]);
        match self.variables.get(&varname) {
            Some(Variable::Function(function)) => function(&self.variables, param),
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