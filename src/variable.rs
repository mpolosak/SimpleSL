use std::collections::HashMap;
use std::fmt;
use crate::params::ParamVec;

type Function = fn(&mut VariableMap, ParamVec) -> Result<Variable, String>;
type Array = Vec<Variable>;

#[derive(Clone)]
pub enum Variable{
    Float(f64),
    Text(String),
    Function(Function),
    Array(Array),
    Referance(String),
    Null
}

pub type VariableMap = HashMap<String, Variable>;

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Float(value)=>write!(f, "{}", value),
            Variable::Text(value)=>write!(f, "{}", value),
            Variable::Function(_)=>write!(f, "Function"),
            Variable::Array(array)=>{
                for value in array{
                    if let Err(e) = write!(f, "{}", value){
                        return Err(e);
                    }
                }
                Ok(())
            }
            Variable::Referance(value) => write!(f, "&{}", value),
            Variable::Null=>write!(f, "Null"),
        }
    }
}