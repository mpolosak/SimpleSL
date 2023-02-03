use crate::array::add_array_functions;
use crate::parse::*;
use crate::stdfunctions::*;
use crate::iofunctions::*;
use crate::variable::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::pest::Parser;
use pest::iterators::Pair;
use crate::function::{Function, NativeFunction};

pub struct Intepreter{
    pub variables:  VariableMap
}

impl Intepreter{
    pub fn new() -> Intepreter{ 
        let variables = VariableMap::new();
        let mut intepreter = Intepreter{variables};
        add_io_functions(&mut intepreter.variables);
        add_std_functions(&mut intepreter.variables);
        add_array_functions(&mut intepreter.variables);
        intepreter
    }

    pub fn exec(&mut self, mut line: String) -> Result<Variable, String>{
        line = line.trim().to_string();
        if line.is_empty() {
            return Ok(Variable::Null)
        }
        let Ok(parse) = SimpleSLParser::parse(Rule::line, &line) else {
            return Err(String::from("Syntax error"));
        };
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

    pub fn exec_expression(&mut self, expression: &Pair<Rule>) -> Result<Variable, String>{
        return match expression.as_rule() { 
            Rule::function_call=>{
                let mut inter = expression.clone().into_inner();
                let Some(ident) = inter.next() else {
                    return Err(String::from("Something strange happened"))
                };
                let var_name = ident.as_str();
                let function = self.variables.get_function(var_name)?;
                let mut array = Array::new();
                let Some(args) = inter.next() else {
                    return Err(String::from("Something strange happened"))
                };
                for arg in args.into_inner() {
                    array.push(self.exec_expression(&arg)?);
                }
                function.exec(String::from(var_name), self, array)
            },
            Rule::num => {
                let Ok(value) = expression.as_str().parse::<f64>() else {
                    return Err(String::from("Something strange happened"))
                };
                Ok(Variable::Float(value))
            },
            Rule::ident => {
                let var_name = String::from(expression.as_str());
                let value = self.variables.get(&var_name)?;
                Ok(value)
            },
            Rule::referance => {
                let Some(ident) = expression.clone().into_inner().next() else {
                    return Err(String::from("Something strange happened"))
                };
                let value = String::from(ident.as_str());
                Ok(Variable::Referance(value))
            },
            Rule::text => {
                let Some(ident) = expression.clone().into_inner().next() else {
                    return Err(String::from("Something strange happened"))
                };
                let value = String::from(ident.as_str());
                Ok(Variable::Text(value))
            },
            Rule::array => {
                let inter = expression.clone().into_inner();
                let mut array = Array::new();
                for element in inter {
                    array.push(self.exec_expression(&element)?);
                }
                Ok(Variable::Array(array))
            },
            Rule::null => Ok(Variable::Null),
            _ => Err(String::from("Something strange happened"))
        }
    }

    pub fn load_and_exec(&mut self, path: &str) -> Result<Variable, String>{
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
    pub fn get(&self, name: &str) -> Result<Variable, String>{
        match self.hash_map.get(name) {
            Some(variable) => Ok(variable.clone()),
            _ => Err(format!("Variable {} doesn't exist", name)),
        }
    }
    pub fn get_function(&self, name: &str) -> Result<Box<dyn Function>, String>{
        return match self.get(name)?{
            Variable::Function(function) => Ok(Box::new(function)),
            Variable::NativeFunction(function) => Ok(Box::new(function)),
            _ => Err(format!("{} should be function", name))
        }
    }
    pub fn insert(&mut self, name: &str, variable: Variable){
        self.hash_map.insert(String::from(name), variable);
    }
    pub fn add_native_function(&mut self, name: &str, function: NativeFunction){
        self.insert(name, Variable::NativeFunction(function))
    }
}