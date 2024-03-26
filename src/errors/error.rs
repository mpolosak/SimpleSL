use crate::{parse::Rule, variable::Type, ExecError};
use match_any::match_any;
use std::{fmt, sync::Arc};

#[derive(Debug)]
pub enum Error {
    VariableDoesntExist(Arc<str>),
    WrongType(Arc<str>, Type),
    WrongNumberOfArguments(Box<str>, usize),
    IndexToBig,
    NegativeIndex,
    NegativeLength,
    NegativeExponent,
    CannotBeParsed(Box<str>),
    CannotIndexInto(Type),
    ZeroDivision,
    ZeroModulo,
    OverflowShift,
    MatchNotCovered,
    IO(std::io::Error),
    Parsing(Box<pest::error::Error<Rule>>),
    IntegerOverflow(Box<str>),
    CannotUnescapeString(unescaper::Error),
    CannotDo(&'static str, Type),
    CannotDo2(Type, &'static str, Type),
    WrongReturn {
        function_name: Option<Arc<str>>,
        function_return_type: Type,
        returned: Type,
    },
    ReturnOutsideFunction,
    MissingReturn {
        function_name: Option<Arc<str>>,
        return_type: Type,
    },
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match_any! { (self, other),
            (Self::VariableDoesntExist(l0), Self::VariableDoesntExist(r0))
            | (Self::CannotBeParsed(l0), Self::CannotBeParsed(r0))
            | (Self::CannotIndexInto(l0), Self::CannotIndexInto(r0))
            | (Self::Parsing(l0), Self::Parsing(r0))
            | (Self::IntegerOverflow(l0), Self::IntegerOverflow(r0)) => l0 == r0,
            (Self::WrongType(l0, l1), Self::WrongType(r0, r1))
            | (Self::WrongNumberOfArguments(l0, l1), Self::WrongNumberOfArguments(r0, r1))
            | (Self::CannotDo(l0, l1), Self::CannotDo(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::IO(l0), Self::IO(r0)) | (Self::CannotUnescapeString(l0), Self::CannotUnescapeString(r0)) => {
                l0.to_string() == r0.to_string()
            },
            (Self::CannotDo2(l0, l1, l2), Self::CannotDo2(r0, r1, r2)) => {
                l0 == r0 && l1 == r1 && l2 == r2
            },
            (
                Self::WrongReturn {
                    function_name,
                    function_return_type,
                    returned,
                },
                Self::WrongReturn {
                    function_name: function_name2,
                    function_return_type: function_return_type2,
                    returned: returned2,
                },
            ) => {
                function_name == function_name2
                    && function_return_type == function_return_type2
                    && returned == returned2
            },
            _ => core::mem::discriminant(self) == core::mem::discriminant(other)
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
            Self::NegativeIndex => write!(f, "cannot index with negative value"),
            Self::NegativeLength => write!(f, "length of an array cannot be negative"),
            Self::NegativeExponent => write!(f, "int value cannot be rised to a negative power"),
            Self::CannotBeParsed(text) => {
                write!(f, "{text} cannot be parsed to variable")
            }
            Self::CannotIndexInto(var_type) => {
                write!(f, "Cannot index into value of type {var_type}")
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
            Self::CannotUnescapeString(error) => write!(f, "{error}"),
            Self::CannotDo(op, var_type) => {
                write!(f, "Cannot do {op} {var_type}")
            }
            Self::CannotDo2(var_type1, op, var_type2) => {
                write!(f, "Cannot do {var_type1} {op} {var_type2}")
            }
            Self::WrongReturn {
                function_name,
                function_return_type,
                returned,
            } => {
                write!(
                    f,
                    "Cannot return {returned} from function{}\n\
                    Function{0} declared to return {function_return_type}",
                    function_name
                        .as_deref()
                        .map(|value| format!(" {value}"))
                        .unwrap_or("".into())
                )
            }
            Self::ReturnOutsideFunction => {
                write!(
                    f,
                    "Return statement can only be used inside of function body"
                )
            }
            Self::MissingReturn {
                function_name,
                return_type,
            } => write!(
                f,
                "Function{} declared to return {return_type} may exit without returning any value\n\
                add return statement at the end of the function or change return type of the function to include ()",
                function_name
                    .as_deref()
                    .map(|value| format!(" {value}"))
                    .unwrap_or("".into())
            ),
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

impl From<unescaper::Error> for Error {
    fn from(value: unescaper::Error) -> Self {
        Error::CannotUnescapeString(value)
    }
}

impl From<ExecError> for Error {
    fn from(value: ExecError) -> Self {
        match value {
            ExecError::IndexToBig => Self::IndexToBig,
            ExecError::NegativeIndex => Self::NegativeIndex,
            ExecError::NegativeLength => Self::NegativeLength,
            ExecError::NegativeExponent => Self::NegativeExponent,
            ExecError::ZeroDivision => Self::ZeroDivision,
            ExecError::ZeroModulo => Self::ZeroModulo,
            ExecError::OverflowShift => Self::OverflowShift,
        }
    }
}
