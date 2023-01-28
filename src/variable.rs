use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use crate::pest::Parser;
use pest::iterators::Pair;
use crate::parse::*;
use crate::function::NativeFunction;

pub type Array = Vec<Variable>;

#[derive(Clone)]
pub enum Variable{
    Float(f64),
    Text(String),
    Function(NativeFunction),
    Array(Array),
    Referance(String),
    Null
}

pub type VariableMap = HashMap<String, Variable>;

impl Variable {
    pub fn type_name(&self) -> String {
        match self {
            Variable::Float(_) => String::from("Float"),
            Variable::Text(_) => String::from("Text"),
            Variable::Function(_) => String::from("Function"),
            Variable::Array(_) => String::from("Array"),
            Variable::Referance(_) => String::from("Referance"),
            Variable::Null => String::from("Null"),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Float(value)=>write!(f, "{}", value),
            Variable::Text(value)=>write!(f, "{}", value),
            Variable::Function(_)=>write!(f, "Function"),
            Variable::Array(array)=>{
                write!(f, "{{")?;
                for value in array{
                    write!(f, "{} ", value)?;
                }
                write!(f, "}}")
            }
            Variable::Referance(value) => write!(f, "&{}", value),
            Variable::Null=>write!(f, "NULL"),
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
                write!(f, "Variable::Array(")?;
                for value in array{
                    write!(f, "{}", value)?;
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
        let s = s.trim();
        let Ok(parse) = SimpleSLParser::parse(Rule::var, s) else {
            return Err(format!("{} cannot be parsed to variable", s))
        };
        if parse.as_str() != s {
            Err(String::from("String contains more than one variable"))
        } else {
            let pair_vec: Vec<Pair<Rule>> = parse.collect();
            variable_from_pair(pair_vec[0].clone())
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
    let Ok(parse) = SimpleSLParser::parse(Rule::ident, name) else {
        return false
    };
    parse.as_str() == name
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