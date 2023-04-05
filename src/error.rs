use std::fmt;
use crate::parse::Rule;

#[derive(Debug, PartialEq)]
pub enum Error {
    VariableDoesntExist(String),
    WrongType(String, String),
    WrongNumberOfArguments(String, usize),
    IndexToBig,
    CannotBeParsed(String),
    TooManyVariables,
    Other(String)
}

impl std::error::Error for Error{}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableDoesntExist(var_name)
                => write!(f, "{var_name} doesn't exist"),
            Self::WrongType(var_name, type_name)
                => write!(f,"{var_name} should be {type_name}"),
            Self::WrongNumberOfArguments(name, 0)
                => write!(f, "{name} requires no arguments but some passed"),
            Self::WrongNumberOfArguments(name, num)
                => write!(f, "{name} requires {num} args"),
            Self::IndexToBig => write!(f, "index must be lower than array size"),
            Self::CannotBeParsed(text)
                => write!(f, "{text} cannot be parsed to variable"),
            Self::TooManyVariables
                => write!(f, "String contains more than one variable"),
            Self::Other(value) => write!(f, "{value}")
        }
    }
}

impl From<pest::error::Error<Rule>> for Error{
    fn from(value: pest::error::Error<Rule>) -> Self {
        Error::Other(value.to_string())
    }
}
