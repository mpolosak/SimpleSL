use crate::{
    error::Error,
    function::Function,
    join, join_debug,
    parse::*,
    pest::Parser,
    variable_type::{GetType, Type},
};
use pest::iterators::Pair;
use std::{fmt, rc::Rc, str::FromStr};

pub type Array = Vec<Variable>;

#[derive(Clone)]
pub enum Variable {
    Int(i64),
    Float(f64),
    String(Rc<str>),
    Function(Rc<dyn Function>),
    Array(Rc<Array>),
    Result(Result<Box<Variable>, Box<Variable>>),
    Null,
}

impl GetType for Variable {
    fn get_type(&self) -> Type {
        match self {
            Variable::Int(_) => Type::Int,
            Variable::Float(_) => Type::Float,
            Variable::String(_) => Type::String,
            Variable::Function(function) => function.get_type(),
            Variable::Array(_) => Type::Array,
            Variable::Result(Ok(variable)) => Type::Result {
                ok: variable.get_type().into(),
                error: Type::Any.into(),
            },
            Variable::Result(Err(error)) => Type::Result {
                ok: Type::Any.into(),
                error: error.get_type().into(),
            },
            Variable::Null => Type::Null,
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Int(value) => write!(f, "{value}"),
            Variable::Float(value) => write!(f, "{value}"),
            Variable::String(value) => write!(f, "{value}"),
            Variable::Function(function) => write!(f, "{function}"),
            Variable::Array(array) => write!(f, "{{{}}}", join(array, ", ")),
            Variable::Result(Ok(variable)) => write!(f, "Ok({variable})"),
            Variable::Result(Err(error)) => write!(f, "Err({error})"),
            Variable::Null => write!(f, "NULL"),
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable::Int(value) => write!(f, "Variable::Float({value})"),
            Variable::Float(value) => write!(f, "Variable::Float({value})"),
            Variable::String(value) => write!(f, "Variable::String(\"{value}\")"),
            Variable::Function(function) => write!(f, "Variable::Function({function:?})"),
            Variable::Array(array) => write!(f, "Variable({{{}}})", join_debug(array, ", ")),
            Variable::Result(Ok(variable)) => write!(f, "Variable::Result(Ok({variable:?}))"),
            Variable::Result(Err(error)) => write!(f, "Variable::Result(Err({error:?}))"),
            Variable::Null => write!(f, "Variable::Null"),
        }
    }
}

impl FromStr for Variable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let parse = SimpleSLParser::parse(Rule::var, s)?;
        if parse.as_str() == s {
            let pair_vec: Vec<Pair<Rule>> = parse.collect();
            Self::try_from(pair_vec[0].clone())
        } else {
            Err(Error::TooManyVariables)
        }
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Variable::Int(value1), Variable::Int(value2)) => value1 == value2,
            (Variable::Float(value1), Variable::Float(value2)) => value1 == value2,
            (Variable::String(value1), Variable::String(value2)) => value1 == value2,
            (Variable::Function(value1), Variable::Function(value2)) => Rc::ptr_eq(value1, value2),
            (Variable::Array(value1), Variable::Array(value2)) => value1 == value2,
            (Variable::Null, Variable::Null) => true,
            _ => false,
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Variable {
    type Error = Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::int => {
                let value = pair.as_str().trim().parse::<i64>().unwrap();
                Ok(Variable::Int(value))
            }
            Rule::float => {
                let value = pair.as_str().trim().parse::<f64>().unwrap();
                Ok(Variable::Float(value))
            }
            Rule::string => {
                let value = pair.into_inner().next().unwrap().as_str();
                let value = unescaper::unescape(value).unwrap();
                Ok(Variable::String(value.into()))
            }
            Rule::array => {
                let array = pair
                    .into_inner()
                    .map(Self::try_from)
                    .collect::<Result<Array, Error>>()?;
                Ok(Variable::Array(Rc::new(array)))
            }
            Rule::null => Ok(Variable::Null),
            _ => Err(Error::CannotBeParsed(pair.as_str().into())),
        }
    }
}

impl From<i64> for Variable {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<bool> for Variable {
    fn from(value: bool) -> Self {
        Self::Int(value.into())
    }
}

impl From<usize> for Variable {
    fn from(value: usize) -> Self {
        Self::Int(value as i64)
    }
}

impl From<f64> for Variable {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<Rc<str>> for Variable {
    fn from(value: Rc<str>) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Variable {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}

impl From<String> for Variable {
    fn from(value: String) -> Self {
        Self::String(value.into())
    }
}

impl From<Rc<dyn Function>> for Variable {
    fn from(value: Rc<dyn Function>) -> Self {
        Self::Function(value)
    }
}

impl From<Rc<Array>> for Variable {
    fn from(value: Rc<Array>) -> Self {
        Self::Array(value)
    }
}

impl From<Array> for Variable {
    fn from(value: Array) -> Self {
        Self::Array(value.into())
    }
}

impl<T: Into<Variable>, S: Into<Variable>> From<Result<T, S>> for Variable {
    fn from(value: Result<T, S>) -> Self {
        match value {
            Ok(variable) => Self::Result(Ok(Box::new(variable.into()))),
            Err(error) => Self::Result(Err(Box::new(error.into()))),
        }
    }
}

impl From<()> for Variable {
    fn from(_value: ()) -> Self {
        Self::Null
    }
}

pub fn is_correct_variable_name(name: &str) -> bool {
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
    fn check_variable_from_str() {
        use crate::error::Error;
        use crate::variable::Variable;
        use std::str::FromStr;
        assert_eq!(Variable::from_str(" 15"), Ok(Variable::Int(15)));
        assert_eq!(Variable::from_str(" 7.5 "), Ok(Variable::Float(7.5)));
        assert_eq!(Variable::from_str("NULL"), Ok(Variable::Null));
        assert_eq!(
            Variable::from_str(r#""print \"""#),
            Ok(Variable::String(String::from("print \"").into()))
        );
        assert_eq!(
            Variable::from_str(r#""print" """#),
            Err(Error::TooManyVariables)
        );
    }
}
