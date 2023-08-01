use crate::{parse::Rule, variable::Type};
use std::{fmt, rc::Rc};

#[derive()]
pub enum Error {
    VariableDoesntExist(Box<str>),
    TypeDoesntExist(Rc<str>),
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
            Self::TypeDoesntExist(var_name) => {
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableDoesntExist(arg0) => {
                f.debug_tuple("VariableDoesntExist").field(arg0).finish()
            }
            Self::TypeDoesntExist(arg0) => f.debug_tuple("TypeDoesntExist").field(arg0).finish(),
            Self::WrongType(arg0, arg1) => {
                f.debug_tuple("WrongType").field(arg0).field(arg1).finish()
            }
            Self::OperandsMustBeBothIntOrBothFloat(arg0) => f
                .debug_tuple("OperandsMustBeBothIntOrBothFloat")
                .field(arg0)
                .finish(),
            Self::BothOperandsMustBeInt(arg0) => {
                f.debug_tuple("BothOperandsMustBeInt").field(arg0).finish()
            }
            Self::OperandMustBeInt(arg0) => f.debug_tuple("OperandMustBeInt").field(arg0).finish(),
            Self::WrongNumberOfArguments(arg0, arg1) => f
                .debug_tuple("WrongNumberOfArguments")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::IndexToBig => write!(f, "IndexToBig"),
            Self::CannotAdd(arg0, arg1) => {
                f.debug_tuple("CannotAdd").field(arg0).field(arg1).finish()
            }
            Self::CannotBeNegative(arg0) => f.debug_tuple("CannotBeNegative").field(arg0).finish(),
            Self::CannotBeParsed(arg0) => f.debug_tuple("CannotBeParsed").field(arg0).finish(),
            Self::CannotIndexInto(arg0) => f.debug_tuple("CannotIndexInto").field(arg0).finish(),
            Self::TooManyVariables => write!(f, "TooManyVariables"),
            Self::ZeroDivision => write!(f, "ZeroDivision"),
            Self::ZeroModulo => write!(f, "ZeroModulo"),
            Self::OverflowShift => write!(f, "OverflowShift"),
            Self::MatchNotCovered => write!(f, "MatchNotCovered"),
            Self::IO(arg0) => f.debug_tuple("IO").field(arg0).finish(),
            Self::Parsing(arg0) => write!(f, "Parsing({arg0})"),
            Self::ArgumentDoesntContainType => write!(f, "ArgumentDoesntContainType"),
        }
    }
}
