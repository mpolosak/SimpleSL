use std::{collections::HashMap,fs::File,rc::Rc,io::{BufReader, BufRead}};
use pest::iterators::Pair;
use crate::function::{NativeFunction, LangFunction};
use crate::{parse::*,variable::*,error::Error,pest::Parser,stdlib::add_std_lib};
pub struct Intepreter{
    pub variables:  VariableMap
}

impl Intepreter{
    pub fn new() -> Intepreter{ 
        let variables = VariableMap::new();
        let mut intepreter = Intepreter{variables};
        add_std_lib(&mut intepreter.variables);
        intepreter
    }

    pub fn exec(&mut self, mut line: String) -> Result<Variable, Error>{
        line = line.trim().to_string();
        if line.is_empty() {
            return Ok(Variable::Null)
        }
        let parse = SimpleSLParser::parse(Rule::line, &line)?;
        let pair_vec: Vec<Pair<Rule>> = parse.collect();
        if pair_vec.len()==3{
            let var = pair_vec[0].as_str();
            let result = self.exec_expression(&pair_vec[1])?;
            self.variables.insert(var, result);
            Ok(Variable::Null)
        } else {
            self.exec_expression(&pair_vec[0])
        }
    }

    pub fn exec_expression(&mut self, expression: &Pair<Rule>) -> Result<Variable, Error>{
        return match expression.as_rule() { 
            Rule::function_call=>{
                let mut inter = expression.clone().into_inner();
                let ident = inter.next().unwrap();
                let var_name = ident.as_str();
                let Variable::Function(function) = self.variables.get(var_name)? else {
                    return Err(Error::WrongType(String::from(var_name), String::from("Function")));
                };
                let mut array = Array::new();
                let args = inter.next().unwrap();
                for arg in args.into_inner() {
                    array.push(self.exec_expression(&arg)?);
                }
                function.exec(String::from(var_name), self, array)
            },
            Rule::num => {
                let value = expression.as_str().parse::<f64>().unwrap();
                Ok(Variable::Float(value))
            },
            Rule::ident => {
                let var_name = String::from(expression.as_str());
                let value = self.variables.get(&var_name)?;
                Ok(value)
            },
            Rule::string => {
                let ident = expression.clone().into_inner().next().unwrap();
                let value = ident.as_str();
                Ok(Variable::String(value.into()))
            },
            Rule::array => {
                let inter = expression.clone().into_inner();
                let mut array = Array::new();
                for element in inter {
                    array.push(self.exec_expression(&element)?);
                }
                Ok(Variable::Array(array.into()))
            },
            Rule::null => Ok(Variable::Null),
            Rule::function => {
                let function = LangFunction::new(&self.variables, expression.clone())?;
                Ok(Variable::Function(Rc::new(function)))
            }
            _ => panic!()
        }
    }

    pub fn load_and_exec(&mut self, path: &str) -> Result<Variable, Error>{
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);
        let mut result = Variable::Null;
        for line in buf_reader.lines() {
            let text = line.unwrap();
            result = self.exec(text)?;
        }
        Ok(result)
    }
}

pub struct VariableMap {
    hash_map: HashMap<String, Variable>
}

impl VariableMap{
    pub fn new() -> Self {
        VariableMap { hash_map: HashMap::new() }
    }
    pub fn get(&self, name: &str) -> Result<Variable, Error>{
        match self.hash_map.get(name) {
            Some(variable) => Ok(variable.clone()),
            _ => Err(Error::VariableDoesntExist(String::from(name))),
        }
    }
    pub fn insert(&mut self, name: &str, variable: Variable){
        self.hash_map.insert(String::from(name), variable);
    }
    pub fn add_native_function(&mut self, name: &str, function: NativeFunction){
        self.insert(name, Variable::Function(Rc::new(function)))
    }
}