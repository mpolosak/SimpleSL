use std::fmt;
use crate::parse::Rule;

#[derive(Debug)]
pub enum Error {
    SomethingStrange,
    VariableDoesntExist(String),
    WrongType(String, String),
    Other(String)
}

impl std::error::Error for Error{}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SomethingStrange => write!(f, "Something strange happend"),
            Self::VariableDoesntExist(var_name)
                => write!(f, "{var_name} doesn't exist"),
            Self::WrongType(var_name, type_name)
                => write!(f,"{var_name} should be {type_name}"),
            Self::Other(value) => write!(f, "{value}")
        }
    }
}

impl From<pest::error::Error<Rule>> for Error{
    fn from(value: pest::error::Error<Rule>) -> Self {
        Error::Other(value.to_string())
    }
}
