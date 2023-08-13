pub mod function_type;
pub mod type_set;
mod variable_type;
use crate::{function::Function, join_debug, parse::*, pest::Parser, Error, Result};
use pest::iterators::Pair;
use std::{fmt, io, rc::Rc, str::FromStr};
pub use variable_type::{GetReturnType, GetType, Type};

#[derive(Clone)]
pub enum Variable {
    Int(i64),
    Float(f64),
    String(Rc<str>),
    Function(Rc<Function>),
    Array(Rc<[Variable]>, Rc<Type>),
    Tuple(Rc<[Variable]>),
    Void,
}

impl GetType for Variable {
    fn get_type(&self) -> Rc<Type> {
        match self {
            Variable::Int(_) => Type::Int.into(),
            Variable::Float(_) => Type::Float.into(),
            Variable::String(_) => Type::String.into(),
            Variable::Function(function) => function.get_type(),
            Variable::Array(_, var_type) => var_type.clone(),
            Variable::Tuple(elements) => {
                let types = elements.iter().map(Variable::get_type).collect();
                Type::Tuple(types).into()
            }
            Variable::Void => Type::Void.into(),
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
            Variable::Array(array, _) => {
                write!(f, "[{}]", join_debug(array, ", "))
            }
            Variable::Tuple(elements) => write!(f, "({})", join_debug(elements, ", ")),
            Variable::Void => write!(f, "()"),
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(value) => write!(f, "{value:?}"),
            Self::Float(value) => write!(f, "{value:?}"),
            Self::String(value) => write!(f, "{value:?}"),
            other => write!(f, "{other}"),
        }
    }
}

impl FromStr for Variable {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let s = s.trim();
        let parse = SimpleSLParser::parse(Rule::var, s)?;
        if parse.as_str() == s {
            let pairs: Box<[Pair<Rule>]> = parse.collect();
            Self::try_from(pairs[0].clone())
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
            (Variable::Array(value1, _), Variable::Array(value2, _)) => value1 == value2,
            (Variable::Void, Variable::Void) => true,
            _ => false,
        }
    }
}

impl TryFrom<Pair<'_, Rule>> for Variable {
    type Error = Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self> {
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
                    .collect::<Result<Rc<[Variable]>>>()?;

                Ok(Variable::from(array))
            }
            Rule::void => Ok(Variable::Void),
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

impl From<Function> for Variable {
    fn from(value: Function) -> Self {
        Self::Function(value.into())
    }
}

impl From<Rc<Function>> for Variable {
    fn from(value: Rc<Function>) -> Self {
        Self::Function(value)
    }
}

impl From<Rc<[Variable]>> for Variable {
    fn from(value: Rc<[Variable]>) -> Self {
        let mut iter = value.iter();
        let var_type = if let Some(first) = iter.next() {
            let mut element_type = first.get_type();
            for instruction in iter {
                element_type = element_type.concat(instruction.get_type().as_ref()).into();
            }
            Type::Array(element_type)
        } else {
            Type::EmptyArray
        };
        Self::Array(value, var_type.into())
    }
}

impl From<Vec<Variable>> for Variable {
    fn from(value: Vec<Variable>) -> Self {
        Rc::<[Self]>::from(value).into()
    }
}

impl From<()> for Variable {
    fn from(_value: ()) -> Self {
        Self::Void
    }
}

impl<T: Into<Variable>> From<Option<T>> for Variable {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => Variable::Void,
        }
    }
}

impl<T: Into<Variable>> From<io::Result<T>> for Variable {
    fn from(value: io::Result<T>) -> Self {
        match value {
            Ok(value) => value.into(),
            Err(error) => error.into(),
        }
    }
}

impl From<io::Error> for Variable {
    fn from(value: io::Error) -> Self {
        Variable::Tuple(Rc::from([value.kind().into(), value.to_string().into()]))
    }
}

impl From<io::ErrorKind> for Variable {
    fn from(value: io::ErrorKind) -> Self {
        (value as i64).into()
    }
}

impl FromIterator<Variable> for Variable {
    fn from_iter<T: IntoIterator<Item = Variable>>(iter: T) -> Self {
        let elements: Rc<[Variable]> = iter.into_iter().collect();
        elements.into()
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
        use crate::variable::Variable;
        use crate::Error;
        use std::str::FromStr;
        assert_eq!(Variable::from_str(" 15"), Ok(Variable::Int(15)));
        assert_eq!(Variable::from_str(" 7.5 "), Ok(Variable::Float(7.5)));
        assert_eq!(Variable::from_str("()"), Ok(Variable::Void));
        assert_eq!(
            Variable::from_str(r#""print \"""#),
            Ok(Variable::String("print \"".into()))
        );
        assert_eq!(
            Variable::from_str(r#""print" """#),
            Err(Error::TooManyVariables)
        );
    }
}
