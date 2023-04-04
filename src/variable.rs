use std::fmt::{self};
use std::rc::Rc;
use std::str::FromStr;
use crate::pest::Parser;
use pest::iterators::Pair;
use crate::parse::*;
use crate::function::Function;

pub type Array = Vec<Variable>;

#[derive(Clone)]
pub enum Variable{
    Float(f64),
    String(Rc<str>),
    Function(Rc<dyn Function>),
    Array(Rc<Array>),
    Null
}

impl Variable {
    pub fn type_name(&self) -> &str {
        match self {
            Variable::Float(_) => "float",
            Variable::String(_) => "string",
            Variable::Function(_) => "function",
            Variable::Array(_) => "array",
            Variable::Null => "null",
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Float(value)=>write!(f, "{value}"),
            Variable::String(value)=>write!(f, "{value}"),
            Variable::Function(function)=>write!(f, "{function}"),
            Variable::Array(array)=>{
                write!(f, "{{")?;
                if let [elements @ .., last] = &array[..] {
                    for var in elements{
                        write!(f, "{var}, ")?;
                    }
                    write!(f, "{last}")?
                }
                write!(f, "}}")
            },
            Variable::Null=>write!(f, "NULL"),
        }
    }
}


impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Float(value)=>write!(f, "Variable::Float({value})"),
            Variable::String(value)=>write!(f, "Variable::Text(\"{value}\")"),
            Variable::Function(function)=>{
                write!(f, "Variable::Function(")?;
                if let [params @ .., last] = &function.get_params()[..] {
                    for param in params{
                        write!(f, "{param}, ")?;
                    }
                    write!(f, "{last}")?;
                }
                write!(f, ")")
            },
            Variable::Array(array)=>{
                write!(f, "Variable({{")?;
                if let [elements @ .., last] = &array[..] {
                    for var in elements{
                        write!(f, "{var:?}, ")?;
                    }
                    write!(f, "{last:?}")?
                }
                write!(f, "}})")
            },
            Variable::Null=>write!(f, "Variable::Null"),
        }
    }
}

impl FromStr for Variable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let Ok(parse) = SimpleSLParser::parse(Rule::var, s) else {
            return Err(format!("{s} cannot be parsed to variable"))
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
                match other {
                    Variable::Float(value2) => value1 == value2,
                    _ => false
                } 
            }
            Variable::String(value1) => {
                match other {
                    Variable::String(value2) => value1 == value2,
                    _ => false
                }
            }
            Variable::Function(value1) => match other{
                Variable::Function(value2) => Rc::ptr_eq(value1, value2),
                _ => false
            },
            Variable::Array(array1) => {
                match  other {
                    Variable::Array(array2) => array1 == array2,
                    _ => false
                }
            },
            Variable::Null => {
                matches!(other, Variable::Null)
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
        assert_eq!(Variable::from_str("NULL"), Ok(Variable::Null));
        assert_eq!(Variable::from_str(r#""print \"""#), Ok(Variable::String(String::from("print \"").into())));
        assert_eq!(Variable::from_str(r#""print" """#),
            Err(String::from("String contains more than one variable")));
        assert_eq!(Variable::from_str("\"print"), Err(String::from("Mismatching quotation marks")));
    }
}