use crate::error::Error;
use crate::variable::*;
use crate::intepreter::{Intepreter, VariableMap};
use std::fmt;
use std::vec::Vec;
use std::iter::zip;
use pest::iterators::Pair;
use crate::parse::Rule;

#[derive(Clone,Debug)]
pub struct Param {
    pub name: String,
    pub type_name: String,
}

impl Param {
    pub fn new(name: &str, type_name: &str) -> Self {
        Param {
            name: String::from(name),
            type_name: String::from(type_name)
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.type_name == "..." {
            write!(f, "{}...", self.name)
        } else {
            write!(f, "{}:{}", self.name, self.type_name)
        }
    }
}

pub trait Function{
    fn exec(&self, name: String, intepreter: &mut Intepreter, mut args: Array)
        -> Result<Variable, Error>{
        let mut args_map = VariableMap::new();
        let params = self.get_params();
        if let Some(Param {name: param_name, type_name}) = &params.last(){
            if *type_name == "..."{
                let from = params.len()-1;
                let rest: Array = args.drain(from..).collect();
                args_map.insert(param_name, Variable::Array(rest));
            } else if args.len() != params.len() {
                return Err(Error::WrongNumberOfArguments(name, params.len()))
            }
        } else if !args.is_empty() {
            return Err(Error::WrongNumberOfArguments(name, 0))
        }

        for (arg, param) in zip(args, params) {
            if param.type_name == "any" || arg.type_name() == param.type_name {
                args_map.insert(&param.name, arg);
            } else {
                return Err(Error::WrongType(param.name.clone(), param.type_name.clone()))
            }
        }
        self.exec_intern(name, intepreter, args_map)
    }
    fn exec_intern(&self, name: String, intepreter: &mut Intepreter,
        args: VariableMap) -> Result<Variable, Error>;
    fn get_params(&self) -> &Vec<Param>;
}

impl fmt::Display for dyn Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function(")?;
        if let [params @ .., last] = &self.get_params()[..] {
            for param in params{
                write!(f, "{param}, ")?;
            }
            write!(f, "{last}")?;
        }
        write!(f, ")")
    }
}

#[derive(Clone)]
pub struct NativeFunction {
    pub params: Vec<Param>,
    pub body: fn(String, &mut Intepreter, VariableMap) -> Result<Variable, Error>,
}

impl Function for NativeFunction {
    fn exec_intern(&self, name: String, intepreter: &mut Intepreter,
        args: VariableMap) -> Result<Variable, Error>{
        (self.body)(name, intepreter, args)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
}

#[derive(Clone)]
pub struct LangFunction {
    pub params: Vec<Param>,
    pub body: Pair<'static, Rule>
}

impl Function for LangFunction {
    fn exec_intern(&self, _name: String, intepreter: &mut Intepreter,
            _args: VariableMap) -> Result<Variable, Error> {
        intepreter.exec_expression(&self.body)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
}