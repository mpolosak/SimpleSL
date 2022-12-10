use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;
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

pub fn is_correct_variable_name(name: &str)->bool{
    lazy_static! {
        static ref RE: Regex = Regex::new("[A-z_][0-9A-z_]*").unwrap();
    }
    let Some(caps) = RE.captures(name) else { return false };
    caps[0]==*name
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_is_correct_variable_name() {
        use crate::variable::is_correct_variable_name;
        assert!(is_correct_variable_name("aDd"));
        assert!(is_correct_variable_name("Ad_d5"));
        assert!(is_correct_variable_name("_ad_d"));
        assert!(!is_correct_variable_name("5add"));
        assert!(!is_correct_variable_name("^$$ddd"));
        assert!(!is_correct_variable_name(""));
        assert!(!is_correct_variable_name("12"));
        assert!(!is_correct_variable_name("%"));

    }
}