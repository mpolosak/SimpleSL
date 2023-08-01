use crate::{parse::Rule, variable::Type};
use std::{fmt, rc::Rc};

#[derive(Debug)]
pub enum Error {
    VariableDoesntExist(Box<str>),
    WrongType(Rc<str>, Type),
    OperandsMustBeBothIntOrBothFloat(&'static str),
    BothOperandsMustBeInt(&'static str),
    OperandMustBeInt(&'static str),
    WrongNumberOfArguments(Box<str>, usize),
    IndexToBig,
    CannotAdd(Type, Type),
    CannotBeNegative(&'static str),
    CannotBeParsed(Box<str>),
    CannotIndexInto(Type),
    TooManyVariables,
    ZeroDivision,
    ZeroModulo,
    OverflowShift,
    MatchNotCovered,
    IO(std::io::Error),
    Parsing(Box<pest::error::Error<Rule>>),
    ArgumentDoesntContainType,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableDoesntExist(var_name) => {
                write!(f, "{var_name} doesn't exist")
            }
            Self::WrongType(var_name, var_type) => {
                write!(f, "{var_name} should be {var_type}")
            }
            Self::OperandsMustBeBothIntOrBothFloat(operator) => {
                write!(
                    f,
                    "Arguments of {operator} operator must be both int or both float",
                )
            }
            Self::BothOperandsMustBeInt(operator) => {
                write!(f, "Both arguments of {operator} operator must be int",)
            }
            Self::OperandMustBeInt(operator) => {
                write!(f, "Argument of {operator} operator must be int",)
            }
            Self::WrongNumberOfArguments(name, 0) => {
                write!(f, "{name} requires no arguments but some passed")
            }
            Self::WrongNumberOfArguments(name, num) => {
                write!(f, "{name} requires {num} args")
            }
            Self::IndexToBig => write!(f, "index must be lower than array size"),
            Self::CannotAdd(type1, type2) => write!(f, "Cannot add {type1} and {type2}"),
            Self::CannotBeNegative(ident) => write!(f, "{ident} cannot be negative"),
            Self::CannotBeParsed(text) => {
                write!(f, "{text} cannot be parsed to variable")
            }
            Self::CannotIndexInto(var_type) => {
                write!(f, "Cannot index into value of type {var_type}")
            }
            Self::TooManyVariables => {
                write!(f, "String contains more than one variable")
            }
            Self::ZeroDivision => {
                write!(f, "Cannot divide by 0")
            }
            Self::ZeroModulo => {
                write!(f, "Cannot calculate the remainder with a divisor of 0")
            }
            Self::OverflowShift => {
                write!(f, "Cannot shift with overflow")
            }
            Self::MatchNotCovered => {
                write!(f, "All posible values must be covered in match")
            }
            Self::IO(error) => write!(f, "{error}"),
            Self::Parsing(error) => write!(f, "{error}"),
            Self::ArgumentDoesntContainType => write!(f, "Argument doesn't contain type"),
        }
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Error::Parsing(value.into())
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
