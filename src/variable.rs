mod array;
mod function_type;
mod type_set;
mod variable_type;
use crate::{function::Function, join_debug, parse::*, Error, Result};
use pest::{iterators::Pair, Parser};
use std::{
    fmt::{self},
    io,
    rc::Rc,
    str::FromStr,
};
pub use variable_type::{ReturnType, Type, Typed};
pub use {array::Array, function_type::FunctionType, type_set::TypeSet};

#[derive(Clone)]
pub enum Variable {
    Int(i64),
    Float(f64),
    String(Rc<str>),
    Function(Rc<Function>),
    Array(Rc<Array>),
    Tuple(Rc<[Variable]>),
    Void,
}

impl Typed for Variable {
    fn as_type(&self) -> Type {
        match self {
            Variable::Int(_) => Type::Int,
            Variable::Float(_) => Type::Float,
            Variable::String(_) => Type::String,
            Variable::Function(function) => function.as_type(),
            Variable::Array(array) => array.as_type(),
            Variable::Tuple(elements) => {
                let types = elements.iter().map(Variable::as_type).collect();
                Type::Tuple(types)
            }
            Variable::Void => Type::Void,
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
            Variable::Array(array) => write!(f, "{array}"),
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
            (Variable::Array(value1), Variable::Array(value2)) => value1 == value2,
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
                let Ok(value) = pair.as_str().trim().parse::<i64>() else {
                    return Err(Error::CannotBeParsed(pair.as_str().into()));
                };
                Ok(Variable::Int(value))
            }
            Rule::float => {
                let Ok(value) = pair.as_str().trim().parse::<f64>() else {
                    return Err(Error::CannotBeParsed(pair.as_str().into()));
                };
                Ok(Variable::Float(value))
            }
            Rule::string => {
                let value = pair.into_inner().next().unwrap().as_str();
                let Ok(value) = unescaper::unescape(value) else {
                    return Err(Error::CannotBeParsed(format!("\"{value}\"").into()));
                };
                Ok(Variable::String(value.into()))
            }
            Rule::array => pair
                .into_inner()
                .map(|pair| pair.into_inner().next().unwrap())
                .map(Self::try_from)
                .collect::<Result<Variable>>(),
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

impl From<()> for Variable {
    fn from(_value: ()) -> Self {
        Self::Void
    }
}

impl<T: Into<Variable>> From<Option<T>> for Variable {
    fn from(value: Option<T>) -> Self {
        value.map_or(Variable::Void, Into::into)
    }
}

impl<T: Into<Variable>> From<io::Result<T>> for Variable {
    fn from(value: io::Result<T>) -> Self {
        value.map_or_else(Into::into, Into::into)
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
        Array::from_iter(iter).into()
    }
}

impl From<Array> for Variable {
    fn from(value: Array) -> Self {
        Variable::Array(value.into())
    }
}

impl From<Rc<Array>> for Variable {
    fn from(value: Rc<Array>) -> Self {
        Variable::Array(value)
    }
}

impl From<Rc<[Variable]>> for Variable {
    fn from(value: Rc<[Variable]>) -> Self {
        Array::from(value).into()
    }
}

impl From<Vec<Variable>> for Variable {
    fn from(value: Vec<Variable>) -> Self {
        Array::from(value).into()
    }
}

pub fn is_correct_variable_name(name: &str) -> bool {
    let Ok(parse) = SimpleSLParser::parse(Rule::ident, name) else {
        return false;
    };
    parse.as_str() == name
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::variable::{Array, Type};

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
        assert_eq!(
            Variable::from_str("[]"),
            Ok(Variable::Array(
                Array {
                    var_type: Type::EmptyArray,
                    elements: Rc::new([])
                }
                .into()
            ))
        );
        assert_eq!(
            Variable::from_str(r#"[4.5, 3, "a", []]"#),
            Ok(Variable::from(Rc::<[Variable]>::from([
                Variable::Float(4.5),
                Variable::Int(3),
                Variable::String("a".into()),
                Variable::from(Rc::<[Variable]>::from([]))
            ])))
        );
        assert_eq!(
            Variable::from_str(r#"[[4.5, []]]"#),
            Ok(Variable::from(Rc::<[Variable]>::from([Variable::from(
                Rc::<[Variable]>::from([
                    Variable::Float(4.5),
                    Variable::from(Rc::<[Variable]>::from([]))
                ])
            ),])))
        )
    }
}
