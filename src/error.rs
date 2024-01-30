use crate::{parse::Rule, variable::Type};
use std::{fmt, rc::Rc};

#[derive(Debug)]
pub enum Error {
    VariableDoesntExist(Rc<str>),
    WrongType(Rc<str>, Type),
    WrongNumberOfArguments(Box<str>, usize),
    IndexToBig,
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
    IntegerOverflow(Box<str>),
    ArgumentDoesntContainType,
    CannotDo(&'static str, Type),
    CannotDo2(Type, &'static str, Type),
    WrongReturn(Type, Type),
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::VariableDoesntExist(l0), Self::VariableDoesntExist(r0)) => l0 == r0,
            (Self::WrongType(l0, l1), Self::WrongType(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::WrongNumberOfArguments(l0, l1), Self::WrongNumberOfArguments(r0, r1)) => {
                l0 == r0 && l1 == r1
            }
            (Self::CannotBeNegative(l0), Self::CannotBeNegative(r0)) => l0 == r0,
            (Self::CannotBeParsed(l0), Self::CannotBeParsed(r0)) => l0 == r0,
            (Self::CannotIndexInto(l0), Self::CannotIndexInto(r0)) => l0 == r0,
            (Self::IO(l0), Self::IO(r0)) => l0.to_string() == r0.to_string(),
            (Self::Parsing(l0), Self::Parsing(r0)) => l0 == r0,
            (Self::IntegerOverflow(l0), Self::IntegerOverflow(r0)) => l0 == r0,
            (Self::CannotDo(l0, l1), Self::CannotDo(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::CannotDo2(l0, l1, l2), Self::CannotDo2(r0, r1, r2)) => {
                l0 == r0 && l1 == r1 && l2 == r2
            }
            (Self::WrongReturn(l0, l1), Self::WrongReturn(r0, r1)) => l0 == r0 && l1 == r1,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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
            Self::WrongNumberOfArguments(name, 0) => {
                write!(f, "{name} requires no arguments but some passed")
            }
            Self::WrongNumberOfArguments(name, num) => {
                write!(f, "{name} requires {num} args")
            }
            Self::IndexToBig => write!(f, "index must be lower than array size"),
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
            Self::IntegerOverflow(value) => write!(f, "{value} is to big too fit in int type"),
            Self::ArgumentDoesntContainType => write!(f, "Argument doesn't contain type"),
            Self::CannotDo(op, var_type) => {
                write!(f, "Cannot do {op} {var_type}")
            }
            Self::CannotDo2(var_type1, op, var_type2) => {
                write!(f, "Cannot do {var_type1} {op} {var_type2}")
            }
            Self::WrongReturn(expected, returned) => {
                write!(
                    f,
                    "Type {returned} of variable that you want\
                    to return doesn't match declared return type {expected}"
                )
            }
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
