use crate::variable::*;
use crate::intepreter::Intepreter;
use std::vec::Vec;
use std::iter::zip;

#[derive(Clone)]
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

pub trait Function{
    fn exec(&self, name: String, intepreter: &mut Intepreter, mut args: Array)
        -> Result<Variable, String>{
        let mut args_map = VariableMap::new();
        let params = self.get_params();
        if let Some(Param {name: param_name, type_name}) = &params.last(){
            if *type_name == String::from("..."){
                let from = params.len()-1;
                let rest: Array = args.drain(from..).collect();
                args_map.insert(param_name.clone(), Variable::Array(rest));
            } else if args.len() != params.len() {
                return Err(format!("{name} requires {} args", params.len()))
            }
        } else if args.len()!=0 {
            return Err(format!("{name} requires no arguments but some passed"))
        }

        for (arg, param) in zip(args, params) {
            if arg.type_name() == param.type_name {
                args_map.insert(param.name.clone(), arg);
            } else {
                return Err(format!("{} should be {}", param.name, param.type_name))
            }
        }
        self.exec_intern(name, intepreter, args_map)
    }
    fn exec_intern(&self, name: String, intepreter: &mut Intepreter,
        args: VariableMap) -> Result<Variable, String>;
    fn get_params(&self) -> &Vec<Param>;
}

#[derive(Clone)]
pub struct NativeFunction {
    pub params: Vec<Param>,
    pub body: fn(String, &mut Intepreter, VariableMap) -> Result<Variable, String>,
}

impl Function for NativeFunction {
    fn exec_intern(&self, name: String, intepreter: &mut Intepreter,
        args: VariableMap) -> Result<Variable, String>{
        (self.body)(name, intepreter, args)
    }
    fn get_params(&self) -> &Vec<Param> {
        &self.params
    }
}