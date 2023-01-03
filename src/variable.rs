use std::collections::HashMap;
use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use crate::parse::get_text;
use crate::Intepreter;

type Function = fn(&mut Intepreter, Array) -> Result<Variable, String>;
pub type Array = Vec<Variable>;

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
                write!(f, "{{");
                for value in array{
                    if let Err(e) = write!(f, "{} ", value){
                        return Err(e);
                    }
                }
                write!(f, "}}")
            }
            Variable::Referance(value) => write!(f, "&{}", value),
            Variable::Null=>write!(f, "Null"),
        }
    }
}


impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Float(value)=>write!(f, "Variable::Float({})", value),
            Variable::Text(value)=>write!(f, "Variable::Text(\"{}\")", value),
            Variable::Function(_)=>write!(f, "Variable::Function"),
            Variable::Array(array)=>{
                write!(f, "Variable::Array(");
                for value in array{
                    if let Err(e) = write!(f, "{}", value){
                        return Err(e);
                    }
                }
                write!(f, ")")
            }
            Variable::Referance(value) => write!(f, "Variable::Referance(\"{}\")", value),
            Variable::Null=>write!(f, "Variable::Null"),
        }
    }
}

impl FromStr for Variable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = String::from(s.trim());
        if s.starts_with("&"){
            s.remove(0);
            if !is_correct_variable_name(&s){
                Err(format!("{} isn't correct variable name", s))
            } else {
                Ok(Variable::Referance(s))
            }
        } else if s == "Null" {
            Ok(Variable::Null)
        } else if let Ok(value) = s.parse::<f64>(){
            Ok(Variable::Float(value))
        } else if s.starts_with("\"") {
            let result = get_text(&mut s)?;
            if s.len() != 0 {
                return Err(String::from("String contains more than one variable"));
            }
            Ok(Variable::Text(result))
        } else {
            Err(format!("{} cannot be parsed to variable", s))
        }
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Variable::Float(value1) => {
                if let Variable::Float(value2) = other {
                    value1 == value2
                } else {
                    false
                } 
            }
            Variable::Text(value1) => {
                if let Variable::Text(value2) = other {
                    value1 == value2
                } else {
                    false
                }
            }
            Variable::Function(_) => false,
            Variable::Array(array1) => {
                if let Variable::Array(array2) = other {
                    array1 == array2
                } else {
                    false
                }
            }
            Variable::Referance(value1)=>{
                if let Variable::Referance(value2) = other {
                    value1 == value2
                } else {
                    false
                }
            }
            Variable::Null => {
                match other {
                    Variable::Null => true,
                    _ => false
                }
            }
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
    #[test]
    fn check_variable_from_str(){
        use std::str::FromStr;
        use crate::variable::Variable;
        assert_eq!(Variable::from_str(" 15"), Ok(Variable::Float(15.0)));
        assert_eq!(Variable::from_str("Null"), Ok(Variable::Null));
        assert_eq!(Variable::from_str("&print "), Ok(Variable::Referance(String::from("print"))));
        assert_eq!(Variable::from_str(r#""print \"""#), Ok(Variable::Text(String::from("print \""))));
        assert_eq!(Variable::from_str(r#""print" """#),
            Err(String::from("String contains more than one variable")));
        assert_eq!(Variable::from_str("\"print"), Err(String::from("Mismatching quotation marks")));
    }
}